use super::scheduler;

#[derive(Clone)]
pub struct Handle {
    pub(crate) inner: scheduler::Handle,
}
