use crate::runtime::handle::Handle;

pub(crate) struct BlockingSchedule {
    handle: Handle,
}

impl BlockingSchedule {
    pub(crate) fn new(handle: &Handle) -> Self {
        match handle.inner {
            crate::runtime::scheduler::Handle::CurrentThread(_) => {}
        }
        BlockingSchedule {
            handle: handle.clone(),
        }
    }
}
