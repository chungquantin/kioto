mod current_thread;
mod scheduler;
mod shared;

pub use scheduler::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum Handle {
    CurrentThread(Arc<current_thread::Handle>),
}
