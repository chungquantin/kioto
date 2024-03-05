use super::{schedule::Schedule, Task};

/// A task was notified.
#[repr(transparent)]
pub(crate) struct Notified<P>(Task<P>);

// safety: This type cannot be used to touch the task without first verifying
// that the value is on a thread where it is safe to poll the task.
unsafe impl<S: Schedule> Send for Notified<S> {}
unsafe impl<S: Schedule> Sync for Notified<S> {}
