use std::marker::PhantomData;

#[derive(Debug)]
pub(super) struct Task<P> {
    _p: PhantomData<P>,
}

/// A task was notified.
#[repr(transparent)]
pub(crate) struct Notified<P>(Task<P>);
