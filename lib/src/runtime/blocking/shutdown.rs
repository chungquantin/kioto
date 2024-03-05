//! A shutdown channel.
use oneshot;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(super) struct Sender {
    // We need atomic reference counter type for the sender is because
    // one worker thread has one shutdown sender. We need to implement Arc
    // type for this field so it can be thread-safe send
    _tx: Arc<oneshot::Sender<()>>,
}

#[derive(Debug)]
pub(super) struct Receiver {
    rx: oneshot::Receiver<()>,
}

impl Receiver {}
