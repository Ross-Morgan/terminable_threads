use std::any::Any;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, self};
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, self};

use super::traits::{Join, Terminate};


/// A group of threads that can be terminated early using an atomic AtomicBool flag
/// The provided functions must take and use this flag to be terminable

pub struct TerminableThreadGroup<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    /// Thread-safe AtomicBool, signalling whether the functions should terminate
    running: Arc<Mutex<AtomicBool>>,

    /// Vec of functions or closures that take an atomic AtomicBool as an argument
    inner_threads: Vec<JoinHandle<T>>,

    _marker: PhantomData<F>,
}


/// A group of threads that can be terminated early using an atomic AtomicBool flag
/// The provided functions must take and use this flag to be terminable
///
/// This uses a fixed size array instead of a vec
pub struct TerminableThreadGroupArray<T, F, const N: usize>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    /// Thead-safe AtomicBool, signalling whether the functions should terminate
    running: Arc<Mutex<AtomicBool>>,

    /// Array of terminable threads
    inner_threads: [JoinHandle<T>; N],

    _marker: PhantomData<F>,
}


impl<T, F> Join<Vec<T>> for TerminableThreadGroup<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    fn join(self) -> Result<Vec<T>, Box<dyn Any + Send + 'static>> {
        self.inner_threads
            .into_iter()
            .map(|h| h.join())
            .collect()
    }
}


impl<T, F, const N: usize> Join<[T; N]> for TerminableThreadGroupArray<T, F, N>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    fn join(self) -> Result<[T; N], Box<dyn Any + Send + 'static>> {
        let r = self.inner_threads
            .map(|h| h.join().expect(""));

        Ok(r)
    }
}


impl<T, F> Terminate for TerminableThreadGroup<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    fn terminate(&self) {
        if let Ok(b) = self.running.lock() {
            b.store(false, atomic::Ordering::SeqCst);
        }
    }
}

impl<T, F, const N: usize> Terminate for TerminableThreadGroupArray<T, F, N>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    fn terminate(&self) {
        if let Ok(b) = self.running.lock() {
            b.store(false, atomic::Ordering::SeqCst);
        }
    }
}


impl<T, F> TerminableThreadGroup<T, F>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Clone + Send + Sync + 'static,
    T: Send + 'static,
{
    pub fn new(funcs: &[F]) -> Self {
        let arc_atom_bool = Arc::new(Mutex::new(AtomicBool::new(true)));

        let mut arc_clones = Vec::with_capacity(funcs.len());

        for c in arc_clones.iter_mut() {
            *c = Arc::clone(&arc_atom_bool);
        }

        let threads = funcs
            .into_iter()
            .cloned()
            .zip(arc_clones.into_iter())
            .map(|(f, b)| thread::spawn(move || f(b)))
            .collect::<Vec<_>>();

        Self {
            running: arc_atom_bool,
            inner_threads: threads,
            _marker: PhantomData
        }
    }
}


impl<T, F, const N: usize> TerminableThreadGroupArray<T, F, N>
where
    F: FnOnce(Arc<Mutex<AtomicBool>>) -> T,
    F: Clone + Send + Sync + 'static,
    T: Send + 'static,
{
    pub fn new(funcs: [F; N]) -> Self {
        let ref arc_atom_bool = Arc::new(Mutex::new(AtomicBool::new(true)));

        let threads = funcs
            .map(|f| (f, Arc::clone(arc_atom_bool)))
            .map(|(f, b)| thread::spawn(move|| f(b)));

        Self {
            running: Arc::clone(arc_atom_bool),
            inner_threads: threads,
            _marker: PhantomData
        }
    }
}
