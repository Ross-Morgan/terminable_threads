use std::{sync::{atomic::AtomicBool, Arc}, thread::{self, JoinHandle}, marker::PhantomData};

/// A thread that can be terminated



/// A group of threads that can be terminated
///
pub struct TerminableThreadGroup<T, F>
where
    F: FnOnce(Arc<AtomicBool>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    /// Thread-safe bool, signalling whether the functions should terminate
    running: Arc<AtomicBool>,

    /// Vec of functions or closures that take an atomic bool as an argument
    inner_threads: Vec<JoinHandle<T>>,


    _marker: PhantomData<F>,
}

/// A group of threads that can be terminated
/// This uses a fixed size array instead of a vec
pub struct TerminableThreadGroupArray<T, F, const N: usize>
where
    F: Fn(Arc<AtomicBool>) -> T,
{
    /// Thead-safe bool, signalling whether the functions should terminate
    running: Arc<AtomicBool>,

    /// Array of terminable threads
    inner_threads: [F; N],
}
