use super::{schedule::BlockingSchedule, shutdown};
use crate::runtime::{builder::ThreadNameFn, task, types::common::Callback};
use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

/// Thread pool for blocking operations
pub(crate) struct BlockingPool {
    spawner: Spawner,
}

/// Spawner responsible for spawning blocking threads
#[derive(Clone)]
pub(crate) struct Spawner {
    inner: Arc<Inner>,
}

#[derive(Default)]
pub(crate) struct SpawnerMetrics {
    num_threads: AtomicUsize,
    num_idle_threads: AtomicUsize,
    queue_depth: AtomicUsize,
}

impl SpawnerMetrics {
    fn num_threads(&self) -> usize {
        self.num_threads.load(Ordering::Relaxed)
    }

    fn num_idle_threads(&self) -> usize {
        self.num_idle_threads.load(Ordering::Relaxed)
    }

    fn inc_num_threads(&self) {
        self.num_threads.fetch_add(1, Ordering::Relaxed);
    }

    fn dec_num_threads(&self) {
        self.num_threads.fetch_sub(1, Ordering::Relaxed);
    }

    fn inc_num_idle_threads(&self) {
        self.num_idle_threads.fetch_add(1, Ordering::Relaxed);
    }

    fn dec_num_idle_threads(&self) -> usize {
        self.num_idle_threads.fetch_sub(1, Ordering::Relaxed)
    }

    fn inc_queue_depth(&self) {
        self.queue_depth.fetch_add(1, Ordering::Relaxed);
    }

    fn dec_queue_depth(&self) {
        self.queue_depth.fetch_sub(1, Ordering::Relaxed);
    }
}

pub(crate) struct Inner {
    // States shared across worker threads
    shared: Mutex<Shared>,

    /// Spawned threads use this name.
    thread_name: ThreadNameFn,

    /// Spawned thread stack size.
    stack_size: Option<usize>,

    /// Call after a thread starts.
    after_start: Option<Callback>,

    /// Call before a thread stops.
    before_stop: Option<Callback>,

    // Maximum number of threads.
    thread_cap: usize,

    // Customizable wait timeout.
    keep_alive: Duration,

    // Metrics about the pool.
    metrics: SpawnerMetrics,
}

pub(crate) struct Shared {
    queue: VecDeque<Task>,
    num_notify: u32,
    shutdown: bool,
    shutdown_tx: Option<shutdown::Sender>,
    /// Prior to shutdown, we clean up `JoinHandles` by having each timed-out
    /// thread join on the previous timed-out thread. This is not strictly
    /// necessary but helps avoid Valgrind false positives, see
    /// <https://github.com/tokio-rs/tokio/commit/646fbae76535e397ef79dbcaacb945d4c829f666>
    /// for more information.
    last_exiting_thread: Option<std::thread::JoinHandle<()>>,
    /// This holds the `JoinHandles` for all running threads; on shutdown, the thread
    /// calling shutdown handles joining on these.
    worker_threads: HashMap<usize, std::thread::JoinHandle<()>>,
    /// This is a counter used to iterate `worker_threads` in a consistent order (for loom's
    /// benefit).
    worker_thread_index: usize,
}

pub(crate) struct Task {
    task: task::Task<BlockingSchedule>,
}
