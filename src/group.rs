use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

/// A group of threads that can be terminated early using an atomic bool flag
/// The provided functions must take and use this flag to be terminable

pub struct TerminableThreadGroup<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    /// Thread-safe bool, signalling whether the functions should terminate
    running: Arc<Mutex<AtomicBool>>,

    /// Vec of functions or closures that take an atomic bool as an argument
    inner_threads: Vec<JoinHandle<T>>,

    _marker: PhantomData<F>,
}

/// A group of threads that can be terminated early using an atomic bool flag
/// The provided functions must take and use this flag to be terminable
///
/// This uses a fixed size array instead of a vec
pub struct TerminableThreadGroupArray<T, F, const N: usize>
where
    F: Fn(Arc<Mutex<AtomicBool>>) -> T,
{
    /// Thead-safe bool, signalling whether the functions should terminate
    running: Arc<Mutex<AtomicBool>>,

    /// Array of terminable threads
    inner_threads: [JoinHandle<T>; N],

    _marker: PhantomData<F>,
}

// TODO: impl `Join<T>`, `Terminate` for {`TerminableThreadGroup`, `TerminableThreadGroupArray`}