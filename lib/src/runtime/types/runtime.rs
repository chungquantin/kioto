use std::future::Future;

use super::join::JoinHandle;

pub trait TokioRuntime {
    /// Spawn task to the tokio runtime
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;

    /// Runs the provided function on an executor dedicated to blocking operations.
    fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;

    /// Runs a future to completion on the Tokio runtime. This is the
    /// runtime's entry point.
    fn block_on<F: Future>(&self, future: F) -> F::Output;
}
