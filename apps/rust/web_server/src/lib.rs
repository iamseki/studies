pub mod fibonacci;

use std::error::Error;

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidPoolSize(usize),
}

impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolCreationError::InvalidPoolSize(size) => write!(f, "Invalid size: {}", size),
        }
    }
}

impl Error for PoolCreationError {}

pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::InvalidPoolSize(size));
        }

        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
