use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;
use std::thread::JoinHandle;

/// A basic thread manager that can signal all threads to terminate / finish early
///
/// Note that threads will only terminate if the `Arc<AtomicBool>` flag is used
#[derive(Debug)]
pub struct TerminableThreads<T, const N: usize> {
    pub(crate) _threads: [JoinHandle<T>; N],
    pub(crate) _terminate_flag: Arc<AtomicBool>,
}

impl<T, const N: usize> TerminableThreads<T, N> {
    pub fn build() -> (TerminableThreadsBuilder<T, N>, Arc<AtomicBool>) {
        TerminableThreadsBuilder::new()
    }

    /// Signal all threads to terminate and cease operation
    ///
    /// ## Note
    ///
    /// This does not guarantee all threads will terminate, or can be terminated.
    ///
    /// Threads will only terminate if the underlying function checks the flag passed to it.s
    pub fn terminate(&self) {
        self._terminate_flag
            .as_ref()
            .store(true, atomic::Ordering::SeqCst);
    }

    /// Join all threads, optionally signalling termination
    ///
    /// Optional termination signalling is useful because no termination signal
    /// will let allow a function to run until it has finished naturally, while
    /// early termination could stop operations earlier than wanted
    ///
    /// # Returns
    ///
    /// `[Result<T, Error>; N]`
    /// 
    /// An array of length N containing the results of joining each thread
    pub fn join(
        self,
        signal_terminate: bool,
    ) -> [Result<T, Box<dyn Any + Send + 'static>>; N] {
        if signal_terminate {
            self.terminate();
        }

        self._threads.map(JoinHandle::join)
    }
}

/// Basic builder for a terminable thread object
///
/// The builder is necessary to provide the termination flag (`Arc<AtomicBool>`)
/// for threads, that are later provided to the builder, to use.
#[derive(Debug)]
pub struct TerminableThreadsBuilder<T, const N: usize> {
    terminate_flag: Arc<AtomicBool>,
    _marker: PhantomData<T>,
}

impl<T, const N: usize> TerminableThreadsBuilder<T, N> {
    /// Create a new `TeminableThreadBuilder`
    pub fn new() -> (Self, Arc<AtomicBool>) {
        let flag = Arc::new(AtomicBool::new(false));

        (
            Self {
                terminate_flag: Arc::clone(&flag),
                _marker: PhantomData,
            },
            flag,
        )
    }

    /// Transform the builder into a `TerminableThreads<T, N>` struct with the specified threads
    pub fn build_with_threads(self, threads: [JoinHandle<T>; N]) -> TerminableThreads<T, N> {
        TerminableThreads {
            _terminate_flag: self.terminate_flag,
            _threads: threads,
        }
    }
}
