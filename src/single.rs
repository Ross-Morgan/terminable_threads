

pub struct TerminableThreadHandle<T, F>
where
    F: FnOnce(Arc<AtomicBool>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    /// Thread-safe bool, signalling whether the function should terminate
    running: Arc<AtomicBool>,

    inner_thread: JoinHandle<T>,

    _marker: PhantomData<F>
}


impl<T, F> TerminableThreadHandle<T, F>
where
    F: Fn(Arc<AtomicBool>) -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    pub fn spawn(f: F) -> Self
    {
        let arc_bool = Arc::new(AtomicBool::new(true));


        Self {
            running: Arc::clone(&arc_bool),
            inner_thread: thread::spawn(move || f(arc_bool)),
            _marker: PhantomData,
        }
    }
}
