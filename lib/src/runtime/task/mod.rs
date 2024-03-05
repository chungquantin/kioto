mod notified;
mod schedule;

use std::marker::PhantomData;

pub use notified::*;

#[derive(Debug)]
pub(super) struct Task<P> {
    _p: PhantomData<P>,
}
