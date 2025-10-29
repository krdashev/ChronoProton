# ChronoPhoton Architecture

## Overview

ChronoPhoton is designed as a modular, high-performance quantum simulation framework with clear separation of concerns.

## Core Modules

### 1. Core Physics Engine (`src/core/`)

The heart of the simulator, implementing fundamental quantum mechanics:

- **Hamiltonians**: Time-dependent and time-independent operators
- **States**: Ket vectors and density matrices
- **Integrators**: RK4, Magnus expansion methods
- **Floquet Analysis**: Quasi-energy computation
- **Lindblad Dynamics**: Open system evolution
- **Observables**: Expectation value calculation

### 2. GPU Backend (`src/gpu/`)

Hardware acceleration layer:

- **Backend**: Abstraction over wgpu/CUDA/ROCm
- **Kernels**: Optimized compute shaders
- **Memory Management**: Device buffer allocation
- **Batch Execution**: Parallel parameter sweeps

### 3. Simulation Layer (`src/simulation/`)

High-level orchestration:

- **Builder**: Fluent API for configuration
- **Runner**: Execution engine
- **Results**: Data storage and analysis
- **Scheduler**: Job queue management

### 4. Data Management (`src/data/`)

Persistence and I/O:

- **Config**: TOML/YAML parsing
- **Export**: HDF5, CSV output
- **Checkpointing**: State serialization

### 5. User Interface (`src/ui/`)

User interaction:

- **CLI**: Command-line tool
- **GUI**: Real-time visualization with egui

## Data Flow

```
Config File → Builder → Runner → Results → Export
                ↓         ↓
              GPU      Integrator
              Backend    Engine
```

## Extension Points

1. **Custom Hamiltonians**: Implement `Hamiltonian` trait
2. **New Observables**: Implement `Observable` trait
3. **Integrators**: Implement `Integrator` trait
4. **Export Formats**: Extend `Exporter`

## Performance Considerations

- GPU kernels for matrices > 64×64
- Rayon parallelism for CPU operations
- Zero-copy buffer management where possible
- Async I/O for file operations

See `claude.md` for complete specifications.
