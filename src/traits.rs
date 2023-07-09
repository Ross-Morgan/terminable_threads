use std::any::Any;

/// Thread types that can be joined, therefore synchronised to the current thread
pub trait Join<T> {
    /// Synchronise the thread with the current thread by blocking until operation has finished
    fn join(self) -> Result<T, Box<dyn Any + Send + 'static>>;
}

/// Thread types that can teminate their underlying function and join prematurely
pub trait Terminate {
    /// Stop operation of the underlying thread on the next iteration
    fn terminate(&self);
}


impl<T> Join<T> for std::thread::JoinHandle<T> {
    fn join(self) -> Result<T, Box<dyn Any + Send + 'static>> {
        self.join()
    }
}
