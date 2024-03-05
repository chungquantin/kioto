pub mod notified;
mod schedule;

use std::marker::PhantomData;

#[derive(Debug)]
pub(super) struct Task<P> {
    _p: PhantomData<P>,
}
