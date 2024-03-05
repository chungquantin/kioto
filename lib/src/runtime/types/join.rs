use std::marker::PhantomData;

pub struct JoinHandle<T> {
    _p: PhantomData<T>,
}

unsafe impl<T: Send> Send for JoinHandle<T> {}
unsafe impl<T: Send> Sync for JoinHandle<T> {}
