# GPU Programming Guide for ChronoPhoton

## Table of Contents
1. [Introduction](#introduction)
2. [GPU Programming Concepts](#gpu-programming-concepts)
3. [WebGPU/wgpu Basics](#webgpuwgpu-basics)
4. [Complete Example: Vector Addition](#complete-example-vector-addition)
5. [Advanced Example: Matrix-Vector Multiplication](#advanced-example-matrix-vector-multiplication)
6. [Performance Optimization](#performance-optimization)
7. [Debugging GPU Code](#debugging-gpu-code)

---

## Introduction

ChronoPhoton uses **wgpu** (WebGPU in Rust) for GPU acceleration. This is NOT CUDA, but the concepts are similar.

### Why wgpu instead of CUDA?

| Feature | wgpu | CUDA |
|---------|------|------|
| Platforms | NVIDIA, AMD, Intel, Apple | NVIDIA only |
| Backend | Vulkan, Metal, DX12 | Proprietary |
| Language | WGSL (WebGPU Shading Language) | CUDA C++ |
| Rust Integration | Native | FFI required |

---

## GPU Programming Concepts

### 1. The Parallel Execution Model

```
CPU: for i in 0..1000 { process(i) }  // Sequential

GPU: Launch 1000 parallel workers
     Worker 0: process(0)
     Worker 1: process(1)
     ...
     Worker 999: process(999)
     // All run simultaneously!
```

### 2. Memory Hierarchy

```
┌─────────────────────────────────────────┐
│ Host (CPU)                              │
│  ├─ RAM: Large, slow from GPU's view   │
└──┼──────────────────────────────────────┘
   │ PCIe Bus (slow transfer)
   ▼
┌─────────────────────────────────────────┐
│ Device (GPU)                            │
│  ├─ VRAM: Fast, but need explicit copy │
│  ├─ Shared Memory: Ultra-fast, limited │
│  └─ Registers: Per-thread, fastest     │
└─────────────────────────────────────────┘
```

### 3. Work Organization

```
Dispatch (entire job)
└─ Work Groups (e.g., 64 groups)
   └─ Work Items (e.g., 256 items per group)
      └─ Each item is one "thread"

Total threads = 64 × 256 = 16,384
```

### 4. Key Constraints

- **All work items run the SAME code** (SIMT: Single Instruction, Multiple Threads)
- **No dynamic memory allocation** on GPU
- **Must pre-allocate all buffers** before dispatch
- **Synchronization is tricky** (avoid if possible)

---

## WebGPU/wgpu Basics

### Step-by-step Workflow

```rust
// 1. Initialize GPU
let instance = wgpu::Instance::new(...);
let adapter = instance.request_adapter(...).await;
let (device, queue) = adapter.request_device(...).await;

// 2. Create buffers
let input_buffer = device.create_buffer(...);
let output_buffer = device.create_buffer(...);

// 3. Write data to GPU
queue.write_buffer(&input_buffer, 0, &data);

// 4. Create compute pipeline (compile shader)
let shader = device.create_shader_module(...);
let pipeline = device.create_compute_pipeline(...);

// 5. Bind buffers to pipeline
let bind_group = device.create_bind_group(...);

// 6. Dispatch compute work
let mut encoder = device.create_command_encoder(...);
{
    let mut pass = encoder.begin_compute_pass(...);
    pass.set_pipeline(&pipeline);
    pass.set_bind_group(0, &bind_group, &[]);
    pass.dispatch_workgroups(num_workgroups, 1, 1);
}
queue.submit(Some(encoder.finish()));

// 7. Read results back
let slice = output_buffer.slice(..);
slice.map_async(...).await;
let data = slice.get_mapped_range();
```

---

## Complete Example: Vector Addition

Let's implement: `c[i] = a[i] + b[i]` for vectors of length N.

### Rust Host Code

```rust
use wgpu::util::DeviceExt;

pub async fn vector_add_gpu(a: &[f32], b: &[f32]) -> Vec<f32> {
    let n = a.len();

    // 1. Initialize GPU
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find GPU adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Compute Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // 2. Create buffers
    let input_a = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input A"),
        contents: bytemuck::cast_slice(a),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let input_b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input B"),
        contents: bytemuck::cast_slice(b),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output"),
        size: (n * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (n * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // 3. Load shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Vector Add Shader"),
        source: wgpu::ShaderSource::Wgsl(VECTOR_ADD_SHADER.into()),
    });

    // 4. Create pipeline
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Vector Add Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    // 5. Bind buffers
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_a.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: input_b.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });

    // 6. Dispatch compute
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Compute Encoder"),
    });

    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);

        // Launch ceil(n / 256) work groups, each with 256 work items
        let workgroup_size = 256;
        let num_workgroups = (n + workgroup_size - 1) / workgroup_size;
        pass.dispatch_workgroups(num_workgroups as u32, 1, 1);
    }

    // Copy output to staging buffer
    encoder.copy_buffer_to_buffer(
        &output_buffer,
        0,
        &staging_buffer,
        0,
        (n * std::mem::size_of::<f32>()) as u64,
    );

    queue.submit(Some(encoder.finish()));

    // 7. Read back results
    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = slice.get_mapped_range();
    let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

    drop(data);
    staging_buffer.unmap();

    result
}
```

### WGSL Shader Code

```wgsl
// Vector addition compute shader

@group(0) @binding(0) var<storage, read> input_a: array<f32>;
@group(0) @binding(1) var<storage, read> input_b: array<f32>;
@group(0) @binding(2) var<storage, read_write> output: array<f32>;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;

    // Bounds check (in case array size not divisible by 256)
    if (idx < arrayLength(&input_a)) {
        output[idx] = input_a[idx] + input_b[idx];
    }
}
```

### Key Points Explained

1. **`@workgroup_size(256)`**: Each work group has 256 threads
2. **`global_invocation_id`**: Built-in variable giving thread ID
3. **`var<storage, read>`**: Read-only buffer
4. **`var<storage, read_write>`**: Read-write buffer
5. **Bounds check**: Handle cases where N is not a multiple of 256

---

## Advanced Example: Matrix-Vector Multiplication

For quantum simulation, we need: `y = A * x` where A is N×N and x is N×1.

### Naive Approach

```wgsl
@group(0) @binding(0) var<storage, read> matrix: array<f32>;  // N×N flattened
@group(0) @binding(1) var<storage, read> vector: array<f32>;  // N
@group(0) @binding(2) var<storage, read_write> result: array<f32>;  // N
@group(0) @binding(3) var<uniform> params: Params;

struct Params {
    n: u32,  // Matrix dimension
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let row = global_id.x;

    if (row >= params.n) {
        return;
    }

    var sum = 0.0;
    for (var col = 0u; col < params.n; col++) {
        let matrix_idx = row * params.n + col;
        sum += matrix[matrix_idx] * vector[col];
    }

    result[row] = sum;
}
```

### For Complex Numbers

We need to handle `Complex64` (two f64 values: real + imaginary).

```wgsl
struct Complex {
    real: f32,
    imag: f32,
}

fn complex_mul(a: Complex, b: Complex) -> Complex {
    // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
    return Complex(
        a.real * b.real - a.imag * b.imag,
        a.real * b.imag + a.imag * b.real
    );
}

fn complex_add(a: Complex, b: Complex) -> Complex {
    return Complex(a.real + b.real, a.imag + b.imag);
}

@compute @workgroup_size(256)
fn matrix_vector_complex(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let row = global_id.x;

    if (row >= params.n) {
        return;
    }

    var sum = Complex(0.0, 0.0);

    for (var col = 0u; col < params.n; col++) {
        let matrix_idx = row * params.n + col;
        let a = matrix[matrix_idx];
        let b = vector[col];
        sum = complex_add(sum, complex_mul(a, b));
    }

    result[row] = sum;
}
```

---

## Performance Optimization

### 1. Memory Coalescing

**Bad**: Threads access random memory locations
**Good**: Adjacent threads access adjacent memory locations

```wgsl
// GOOD: Thread i accesses element i
let idx = global_id.x;
let value = array[idx];

// BAD: Random access pattern
let idx = some_random_permutation[global_id.x];
let value = array[idx];
```

### 2. Shared Memory for Matrix Multiplication

Use workgroup shared memory to cache data:

```wgsl
var<workgroup> tile: array<Complex, 256>;

@compute @workgroup_size(16, 16)  // 2D workgroup
fn optimized_matmul(@builtin(global_invocation_id) global_id: vec3<u32>,
                    @builtin(local_invocation_id) local_id: vec3<u32>) {
    // Load tile into shared memory
    // Each thread loads one element
    let tx = local_id.x;
    let ty = local_id.y;

    // ... tiling algorithm ...
}
```

### 3. Reduce Host-Device Transfers

```rust
// BAD: Transfer every iteration
for i in 0..1000 {
    upload_to_gpu(data);
    compute_on_gpu();
    download_from_gpu(result);
}

// GOOD: Keep data on GPU
upload_to_gpu(data);
for i in 0..1000 {
    compute_on_gpu();  // Data stays on GPU
}
download_from_gpu(result);
```

### 4. Batch Processing

```rust
// BAD: Process one state at a time
for state in states {
    gpu_evolve(state);
}

// GOOD: Process all states in one kernel launch
gpu_evolve_batch(states);
```

---

## Debugging GPU Code

### 1. Validation Errors

Enable validation layers:

```rust
let (device, queue) = adapter
    .request_device(
        &wgpu::DeviceDescriptor {
            // ... other fields ...
            required_features: wgpu::Features::empty(),
        },
        None,  // Enable validation: Some(std::path::Path::new("trace"))
    )
    .await?;
```

### 2. Print Debugging (Limited)

GPU has no `printf`! Instead:

```wgsl
// Write debug values to a buffer
@group(0) @binding(3) var<storage, read_write> debug_buffer: array<f32>;

fn main() {
    let idx = global_id.x;
    debug_buffer[idx] = some_intermediate_value;
    // ... rest of computation ...
}
```

Then read `debug_buffer` on CPU to inspect values.

### 3. Start Small

```rust
// Test with tiny sizes first
let n = 4;  // Not 1000!
let result = gpu_compute(n);
println!("GPU result: {:?}", result);

// Compare with CPU
let expected = cpu_compute(n);
assert_eq!(result, expected);
```

### 4. Use Render Doc

**RenderDoc** is a graphics debugger that works with wgpu:
- Capture GPU calls
- Inspect buffers
- Step through shader execution

Download: https://renderdoc.org/

---

## ChronoPhoton-Specific Implementation Plan

### Phase 1: Simple Matrix-Vector Multiply

File: `src/gpu/kernels.rs`

```rust
pub async fn matrix_vector_multiply_gpu(
    matrix: &Array2<Complex64>,
    vector: &Array1<Complex64>,
) -> Result<Array1<Complex64>> {
    // 1. Initialize GPU (cache this globally later)
    // 2. Create buffers for matrix, vector, result
    // 3. Upload data
    // 4. Dispatch compute shader
    // 5. Download result
    // 6. Return as Array1
}
```

Shader: `src/gpu/shaders/matrix_vector.wgsl`

### Phase 2: Batched RK4 Integration

```rust
pub async fn rk4_step_batch_gpu(
    states: &[Array1<Complex64>],
    hamiltonians: &[Array2<Complex64>],
    dt: f64,
) -> Result<Vec<Array1<Complex64>>> {
    // Launch one work group per state
    // Each group does RK4 for its state
}
```

### Phase 3: Parameter Sweep Acceleration

```rust
pub async fn parameter_sweep_gpu(
    base_hamiltonian: &dyn Hamiltonian,
    parameter_values: &[f64],
    initial_state: &QuantumState,
) -> Result<Vec<SimulationResults>> {
    // All parameter values computed in parallel on GPU
}
```

---

## Learning Resources

1. **WebGPU Fundamentals**: https://webgpufundamentals.org/
2. **wgpu Tutorial**: https://sotrh.github.io/learn-wgpu/
3. **WGSL Spec**: https://www.w3.org/TR/WGSL/
4. **GPU Gems** (general concepts): https://developer.nvidia.com/gpugems/

---

## Common Pitfalls

### 1. Buffer Alignment

```rust
// GPU buffers must be aligned to specific boundaries
// wgpu handles this automatically for most cases
let size = (data.len() * std::mem::size_of::<f32>()) as u64;
// wgpu rounds up to alignment automatically
```

### 2. Async Initialization

```rust
// WRONG: Can't use .await in non-async context
let backend = GpuBackend::new(true).await;  // Error in sync function!

// RIGHT: Make function async or use block_on
let backend = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(GpuBackend::new(true));
```

### 3. Forgetting Bounds Checks

```wgsl
// ALWAYS check bounds
if (idx >= arrayLength(&array)) {
    return;
}
```

### 4. Wrong Buffer Usage Flags

```rust
// Must include COPY_SRC to copy from GPU → CPU
let buffer = device.create_buffer(&wgpu::BufferDescriptor {
    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    // ^^^ Missing COPY_SRC causes runtime error!
});
```

---

## Next Steps for ChronoPhoton

1. **Implement `GpuBackend::new()`** in `src/gpu/backend.rs`
2. **Create first shader** for matrix-vector multiply
3. **Add benchmark** comparing CPU vs GPU performance
4. **Test with small Hilbert dimensions** (N=10) first
5. **Scale up** to N=1000 once working

Good luck! GPU programming has a steep learning curve, but once you "get it," it's incredibly powerful for parallel workloads like quantum simulation.
