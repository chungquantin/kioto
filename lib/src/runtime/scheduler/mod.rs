mod current_thread;
pub mod scheduler;
mod shared;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum Handle {
    CurrentThread(Arc<current_thread::Handle>),
}
