use std::time::Duration;

use super::{runtime::Runtime, types::common::Callback};

pub(crate) type ThreadNameFn = std::sync::Arc<dyn Fn() -> String + Send + Sync + 'static>;

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    CurrentThread,
}

/// Runtime builder
pub struct Builder {
    kind: Kind,

    /// Cap on thread usage.
    max_blocking_threads: usize,

    /// Interval period polling event
    event_interval: u32,

    /// Customizable keep alive timeout for `BlockingPool`
    pub(super) keep_alive: Option<Duration>,

    /// Name fn used for threads spawned by the runtime.
    pub(super) thread_name: ThreadNameFn,

    pub(super) thread_stack_size: Option<usize>,

    /// Call after a thread starts.
    pub(super) after_start: Option<Callback>,

    /// Call before a thread stops.
    pub(super) before_stop: Option<Callback>,
}

impl Builder {
    pub(crate) fn new(kind: Kind, event_interval: u32) -> Builder {
        Builder {
            kind,
            event_interval,
            // Q: why the max blocking threads is 512?
            max_blocking_threads: 512,
            // Default thread name
            thread_name: std::sync::Arc::new(|| "kioto-runtime-worker".into()),
            keep_alive: None,
            after_start: None,
            before_stop: None,
            thread_stack_size: None,
        }
    }

    pub(super) fn build_current_thread_runtime(&self) -> std::io::Result<Runtime> {
        return Ok(Runtime::new());
    }

    pub fn new_current_thread() -> Builder {
        const EVENT_INTERVAL: u32 = 4;
        Builder::new(Kind::CurrentThread, EVENT_INTERVAL)
    }

    pub fn build(&self) -> std::io::Result<Runtime> {
        match self.kind {
            Kind::CurrentThread => self.build_current_thread_runtime(),
        }
    }
}
