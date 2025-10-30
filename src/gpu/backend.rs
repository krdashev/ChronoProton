use crate::utils::{Error, Result};

#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub name: String,
    pub backend_type: BackendType,
    pub memory_bytes: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum BackendType {
    Cuda,
    Vulkan,
    Metal,
    WebGpu,
    Cpu,
}

pub struct GpuBackend {
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    enabled: bool,
}

impl GpuBackend {
    pub async fn new(enabled: bool) -> Result<Self> {
        if !enabled {
            return Ok(Self {
                device: None,
                queue: None,
                enabled: false,
            });
        }

        tracing::info!("GPU backend requested but not yet implemented");

        Ok(Self {
            device: None,
            queue: None,
            enabled: false,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled && self.device.is_some()
    }

    pub async fn available_devices() -> Result<Vec<GpuDevice>> {
        Ok(vec![GpuDevice {
            name: "CPU Fallback".to_string(),
            backend_type: BackendType::Cpu,
            memory_bytes: 0,
        }])
    }
}

impl Default for GpuBackend {
    fn default() -> Self {
        Self {
            device: None,
            queue: None,
            enabled: false,
        }
    }
}
