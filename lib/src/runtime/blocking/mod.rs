use self::pool::BlockingPool;
use super::builder::Builder;

mod pool;
mod schedule;
mod shutdown;
mod task;

pub(crate) fn create_blocking_pool(builder: &Builder, thread_cap: usize) -> BlockingPool {
    return BlockingPool::new(builder, thread_cap);
}
