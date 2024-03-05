//! A shutdown channel.
use oneshot;
use std::{sync::Arc, time::Duration};

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

pub(super) fn channel() -> (Sender, Receiver) {
    let (tx, rx) = oneshot::channel();
    let tx = Sender { _tx: Arc::new(tx) };
    let rx = Receiver { rx };

    (tx, rx)
}

impl Receiver {
    /// Blocks the current thread until all `Sender` handles drop.
    pub(crate) fn wait(&mut self, timeout: Option<Duration>) {
        unimplemented!()
    }
}
