use crate::utils::Result;

pub struct BatchExecutor {
    batch_size: usize,
    gpu_enabled: bool,
}

impl BatchExecutor {
    pub fn new(batch_size: usize, gpu_enabled: bool) -> Self {
        Self {
            batch_size,
            gpu_enabled,
        }
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn is_gpu_enabled(&self) -> bool {
        self.gpu_enabled
    }

    pub fn execute<F, T>(&self, operations: Vec<F>) -> Result<Vec<T>>
    where
        F: Fn() -> Result<T> + Send,
        T: Send,
    {
        use rayon::prelude::*;

        operations.into_par_iter().map(|op| op()).collect()
    }
}
