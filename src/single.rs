use std::any::Any;
use std::marker::PhantomData;
use std::sync::atomic::{self, AtomicBool};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use super::traits::{Join, Terminate};


/// A thread type that can terminate early using an atomic bool flag
/// The provided function must take and use this flag to be terminable
pub struct TerminableThreadHandle<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    pub(crate) running: Arc<Mutex<AtomicBool>>,
    pub(crate) inner_thread: JoinHandle<T>,
    pub(crate) _marker: PhantomData<F>,
}


impl<T, F> Join<T> for TerminableThreadHandle<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    fn join(self) -> Result<T, Box<dyn Any + Send + 'static>> {
        self.inner_thread.join()
    }
}


impl<T, F> Terminate for TerminableThreadHandle<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    fn terminate(&self) {
        // TODO: Get rid of panic
        match self.running.lock() {
            Ok(b) => b.store(false, atomic::Ordering::SeqCst),
            Err(_) => panic!("Couldn't terminate terminable thread due to mutex already being locked by current thread"),
        };
    }
}
