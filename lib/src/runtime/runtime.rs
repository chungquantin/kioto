use super::{scheduler::scheduler::Scheduler, types::join::JoinHandle};
use std::future::Future;

pub struct Runtime {
    scheduler: Scheduler,
}

impl Runtime {
    pub fn new() -> Self {
        return Runtime {
            scheduler: Scheduler::build_current_threaded_scheduler(),
        };
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        todo!()
    }

    pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        todo!()
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        todo!()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        println!("DROP Runtime");
    }
}

impl std::panic::UnwindSafe for Runtime {}

impl std::panic::RefUnwindSafe for Runtime {}
