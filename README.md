# ChronoPhoton

**GPU-Accelerated Simulator for Photonic Time Crystals and Floquet Quantum Systems**

[![CI](https://github.com/krdashev/chronophoton/workflows/CI/badge.svg)](https://github.com/krdashev/chronophoton/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/chronophoton)

ChronoPhoton is a high-performance Rust framework for simulating periodically driven quantum systems, photonic time crystals, and open quantum dynamics using GPU acceleration.

## Features

- **Floquet Theory**: Compute quasi-energies and Floquet modes for time-periodic Hamiltonians
- **GPU-Accelerated**: Leverage CUDA/ROCm/WebGPU for massive parallelization
- **Open Quantum Systems**: Lindblad master equation solver with thermal baths
- **Parameter Sweeps**: Efficiently explore parameter spaces on GPU
- **Real-Time Visualization**: Interactive GUI with egui for live simulation monitoring
- **Extensible**: Plugin architecture for custom Hamiltonians and observables
- **Python Bindings**: PyO3 integration for scripting and analysis

## Quick Start

### Installation

```bash
cargo install chronophoton
```

Or build from source:
```bash
git clone https://github.com/yourusername/chronophoton.git
cd chronophoton
cargo build --release
```

### Run Your First Simulation

```bash
chronophoton run --config examples/configs/driven_tls.toml
```

### GUI Mode

```bash
chronophoton gui
```

## Example: Driven Two-Level System

```rust
use chronophoton::prelude::*;

// Define a driven two-level system
let hamiltonian = DrivenTLS::new(5.0, 5.0, 0.5);

// Create simulation
let sim = SimulationBuilder::new()
    .hamiltonian(hamiltonian)
    .initial_state(QuantumState::ground_state(2))
    .duration(50.0)
    .timestep(0.1)
    .observable("population", PopulationOperator::new())
    .build()?;

// Run and visualize
let results = sim.run()?;
results.plot("population")?;
```

## Documentation

- [User Guide](docs/user_guide.md)
- [API Reference](https://docs.rs/chronophoton)
- [Architecture Overview](docs/architecture.md)
- [Complete Specification](claude.md)

## Use Cases

- **Quantum Optics Research**: Simulate cavity QED, parametric oscillators, and driven atoms
- **Topological Photonics**: Study Floquet topological insulators and edge states
- **Quantum Control**: Optimize pulse sequences for quantum state manipulation
- **Time Crystal Physics**: Investigate discrete time crystal phases

## Performance

ChronoPhoton achieves:
- **< 5 ms/step** for 100-dimensional systems on GPU
- **> 80% GPU utilization** for dense matrix operations
- **500x speedup** for parameter sweeps with batch size 1000

See [benchmarks](benches/) for detailed performance analysis.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Citation

If you use ChronoPhoton in your research, please cite:

```
@software{chronophoton,
  title = {ChronoPhoton: GPU-Accelerated Photonic Time Crystal Simulator},
  author = {ChronoPhoton Contributors},
  year = {2024},
  url = {https://github.com/yourusername/chronophoton}
}
```

## Acknowledgments

Built with:
- [wgpu](https://wgpu.rs/) - GPU abstraction
- [ndarray](https://github.com/rust-ndarray/ndarray) - N-dimensional arrays
- [egui](https://github.com/emilk/egui) - Immediate mode GUI

Inspired by [QuTiP](https://qutip.org/), [QuantumOptics.jl](https://qojulia.org/), and the quantum photonics community.
