use super::current_thread::CurrentThread;
/// The runtime scheduler is either a multi-thread or a current-thread executor.
/// - phase 1: only support single thread
#[derive(Debug)]
pub(crate) enum Scheduler {
    /// Execute all tasks on the current-thread.
    CurrentThread(CurrentThread),
}

impl Scheduler {
    pub fn build_current_threaded_scheduler() -> Scheduler {
        let (scheduler, _) = CurrentThread::new();
        return Scheduler::CurrentThread(scheduler);
    }
}
