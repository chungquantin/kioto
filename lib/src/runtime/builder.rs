use super::runtime::Runtime;

pub(crate) type ThreadNameFn = std::sync::Arc<dyn Fn() -> String + Send + Sync + 'static>;

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    CurrentThread,
}

/// Runtime builder
pub struct Builder {
    kind: Kind,

    /// Name fn used for threads spawned by the runtime.
    pub(super) thread_name: ThreadNameFn,

    /// Cap on thread usage.
    max_blocking_threads: usize,

    event_interval: u32,
}

impl Builder {
    pub fn new(kind: Kind, event_interval: u32) -> Builder {
        Builder {
            kind,
            event_interval,

            // Q: why the max blocking threads is 512?
            max_blocking_threads: 512,

            // Default thread name
            thread_name: std::sync::Arc::new(|| "kioto-runtime-worker".into()),
        }
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

    pub fn build_current_thread_runtime(&self) -> std::io::Result<Runtime> {
        unimplemented!()
    }
}
