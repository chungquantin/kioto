use core::fmt;
use std::{collections::VecDeque, sync::Arc};

use crate::{
    runtime::task::{self},
    utils::AtomicCell,
};

use super::shared::Shared;

/// Initial queue capacity.
const INITIAL_CAPACITY: usize = 64;

/// Notified struct wrapper around the Task<Arc<Handle>>
/// struct is used to manage the wake status of the task
type NotifiedTask = task::Notified<Arc<Handle>>;

struct SchedulerCore {
    tasks: VecDeque<NotifiedTask>,
}

pub(crate) struct CurrentThread {
    core: AtomicCell<SchedulerCore>,
    notify: Arc<Handle>,
}

/// Handle to the current thread scheduler
#[derive(Debug, Clone)]
pub(crate) struct Handle {
    /// Scheduler state shared across threads
    shared: Shared,
}

impl fmt::Debug for CurrentThread {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("CurrentThread").finish()
    }
}

impl CurrentThread {
    pub fn new() -> (Self, Arc<Handle>) {
        let core = AtomicCell::new(Some(Box::new(SchedulerCore {
            tasks: VecDeque::with_capacity(INITIAL_CAPACITY),
        })));
        let handle = Arc::new(Handle {
            shared: Shared::default(),
        });
        let scheduler = CurrentThread {
            core,
            notify: handle.clone(),
        };
        (scheduler, handle)
    }
}
