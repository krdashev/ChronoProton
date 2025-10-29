//! GPU memory management

use crate::utils::Result;

/// GPU memory buffer wrapper
pub struct GpuBuffer {
    size_bytes: usize,
}

impl GpuBuffer {
    pub fn new(size_bytes: usize) -> Result<Self> {
        Ok(Self { size_bytes })
    }

    pub fn size(&self) -> usize {
        self.size_bytes
    }
}

/// Memory pool for efficient allocation
pub struct GpuMemoryPool {
    total_allocated: usize,
}

impl GpuMemoryPool {
    pub fn new() -> Self {
        Self {
            total_allocated: 0,
        }
    }

    pub fn allocate(&mut self, size_bytes: usize) -> Result<GpuBuffer> {
        self.total_allocated += size_bytes;
        GpuBuffer::new(size_bytes)
    }

    pub fn total_allocated(&self) -> usize {
        self.total_allocated
    }
}

impl Default for GpuMemoryPool {
    fn default() -> Self {
        Self::new()
    }
}
