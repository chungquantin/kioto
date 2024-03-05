use anyhow::Result;

use super::{schedule::BlockingSchedule, shutdown, task::BlockingTask};
use crate::runtime::{
    builder::ThreadNameFn,
    handle::Handle,
    task,
    types::{common::Callback, join::JoinHandle},
    Builder,
};
use core::fmt;
use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

const KEEP_ALIVE: Duration = Duration::from_secs(10);

/// Thread pool for blocking operations
pub(crate) struct BlockingPool {
    spawner: Spawner,
    shutdown_rx: shutdown::Receiver,
}

impl BlockingPool {
    pub(crate) fn new(builder: &Builder, thread_cap: usize) -> BlockingPool {
        let (shutdown_tx, shutdown_rx) = shutdown::channel();
        let keep_alive = builder.keep_alive.unwrap_or(KEEP_ALIVE);

        BlockingPool {
            spawner: Spawner {
                inner: Arc::new(Inner {
                    shared: Mutex::new(Shared {
                        queue: VecDeque::new(),
                        num_notify: 0,
                        shutdown: false,
                        shutdown_tx: Some(shutdown_tx),
                        last_exiting_thread: None,
                        worker_threads: HashMap::new(),
                        worker_thread_index: 0,
                    }),
                    thread_name: builder.thread_name.clone(),
                    stack_size: builder.thread_stack_size,
                    after_start: builder.after_start.clone(),
                    before_stop: builder.before_stop.clone(),
                    thread_cap,
                    metrics: SpawnerMetrics::default(),
                    keep_alive,
                }),
            },
            shutdown_rx,
        }
    }

    pub(crate) fn spawner(&self) -> &Spawner {
        &self.spawner
    }
}

impl Drop for BlockingPool {
    fn drop(&mut self) {
        println!("DROP BlockingPool");
    }
}

impl fmt::Debug for BlockingPool {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("BlockingPool").finish()
    }
}

/// Spawner responsible for spawning blocking threads
#[derive(Clone)]
pub(crate) struct Spawner {
    inner: Arc<Inner>,
}

impl Spawner {
    #[track_caller]
    pub(crate) fn spawn_blocking<F, R>(&self, rt: &Handle, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let fut = BlockingTask::new(func);

        unimplemented!()
    }

    #[track_caller]
    pub(crate) fn spawn_task() {}
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
