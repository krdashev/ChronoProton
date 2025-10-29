# Changelog

All notable changes to ChronoPhoton will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure
- Core physics engine with Floquet solver
- GPU backend using wgpu
- CLI interface
- Example configurations

## [0.1.0] - TBD

### Added
- Floquet theory solver for time-periodic Hamiltonians
- RK4 and Magnus expansion integrators
- Lindblad master equation solver for open quantum systems
- GPU-accelerated matrix operations
- Parameter sweep functionality
- Real-time visualization with egui
- HDF5 export for simulation results
- Example systems: driven two-level, parametric cavity, coupled cavities
- Comprehensive test suite with analytical validation
- Docker container support

### Physics Features
- Time-dependent Hamiltonian support
- Multiple Lindblad operators with thermal baths
- Observable calculation (expectation values, correlators)
- Entanglement entropy computation
- Floquet quasi-energy spectrum analysis

### Performance
- < 5 ms/step for 100-dimensional systems on GPU
- Batch parameter sweeps with 500x speedup
- > 80% GPU utilization for dense operations

[Unreleased]: https://github.com/yourusername/chronophoton/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/chronophoton/releases/tag/v0.1.0
