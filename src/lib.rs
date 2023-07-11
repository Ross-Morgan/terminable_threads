use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, self};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

/// A basic thread manager that can signal all threads to terminate / finish early
/// 
/// Note that threads will only terminate if the `Ref<Mutex<AtomicBool>>` flag is used
#[derive(Debug)]
pub struct TerminableThreads<T, const N: usize> {
    pub(crate) _threads: [JoinHandle<T>; N],
    pub(crate) _terminate_flag: Arc<Mutex<AtomicBool>>,
}

impl<T, const N: usize> TerminableThreads<T, N> {
    pub fn build() -> (TerminableThreadsBuilder<T, N>, Arc<Mutex<AtomicBool>>) {
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
        self._terminate_flag.lock().expect("Failed to lock termination flag").store(true, atomic::Ordering::SeqCst);
    }

    /// Join all threads, optionally signalling termination
    /// 
    /// Optional termination signalling is useful because no termination signal
    /// will let allow a function to run until it has finished naturally, while
    /// early termination could stop operations earlier than wanted
    /// 
    /// # Returns
    /// 
    /// `[T; N]` being the results of each thread.
    /// 
    /// `[Option<Error>; N]` containing only the errors from each thread that
    /// caused an error
    pub fn join(self, signal_terminate: bool) -> Result<[T; N], [Option<Box<dyn Any + Send + 'static>>; N]> {
        if signal_terminate {
            self.terminate();
        }

        let joined = self._threads.map(JoinHandle::join);

        let has_errs = joined.iter().filter(|&r| r.is_err()).next().is_some();

        if !has_errs {
            let res = joined.map(|r| r.unwrap());

            Ok(res)
        } else {
            let res = joined.map(|r| match r {
                    Ok(a) => Err(a),
                    Err(b) => Ok(Some(b)),
            })
            .map(|r| r.unwrap_or(None));

            Err(res)
        }
    }
}


/// Basic builder for a terminable thread object
/// 
/// The builder is necessary to provide the termination flag (`Arc<Mutex<AtomicBool>>`)
/// for threads, that are later provided to the builder, to use.
#[derive(Debug)]
pub struct TerminableThreadsBuilder<T, const N: usize> {
    terminate_flag: Arc<Mutex<AtomicBool>>,
    _marker: PhantomData<T>,
}

impl<T, const N: usize> TerminableThreadsBuilder<T, N> {
    /// Create a new `TeminableThreadBuilder`
    pub fn new() -> (Self, Arc<Mutex<AtomicBool>>) {
        let flag = Arc::new(Mutex::new(AtomicBool::new(false)));

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
            _threads: threads
        }
    }
}
