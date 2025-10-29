# ChronoPhoton: GPU-Accelerated Photonic Time Crystal Simulator

## Table of Contents
1. [Physics Background](#physics-background)
2. [Functional Requirements](#functional-requirements)
3. [Non-Functional Requirements](#non-functional-requirements)
4. [System Architecture](#system-architecture)
5. [APIs and Interfaces](#apis-and-interfaces)
6. [Use Cases and Examples](#use-cases-and-examples)
7. [Testing Methodology](#testing-methodology)
8. [Security and Robustness](#security-and-robustness)
9. [Deployment](#deployment)
10. [Key Performance Indicators](#key-performance-indicators)

---

## 1. Physics Background

### 1.1 Floquet Theory
ChronoPhoton simulates periodically driven quantum systems using **Floquet theory**, which extends time-independent quantum mechanics to time-periodic Hamiltonians:

$$H(t) = H(t + T)$$

The system's time evolution is governed by the **Floquet Hamiltonian** $H_F$, whose eigenstates (Floquet states) and quasi-energies characterize the system's long-term dynamics.

**Key Concepts:**
- **Floquet operator**: $U(T) = \mathcal{T} \exp\left(-i \int_0^T H(t) dt\right)$
- **Quasi-energies**: $\varepsilon_\alpha$ defined modulo $\hbar\omega$ where $\omega = 2\pi/T$
- **Floquet modes**: $|\psi_\alpha(t)\rangle = e^{-i\varepsilon_\alpha t/\hbar}|\phi_\alpha(t)\rangle$

### 1.2 Photonic Time Crystals
Photonic time crystals are systems where the refractive index or permittivity is modulated periodically in time, leading to:
- **Temporal band structure** analogous to spatial photonic crystals
- **Parametric amplification** at specific frequencies
- **Time-reversed analogues** of spatial photonic phenomena

### 1.3 Open Quantum Systems
ChronoPhoton incorporates dissipation and decoherence via the **Lindblad master equation**:

$$\frac{d\rho}{dt} = -\frac{i}{\hbar}[H(t), \rho] + \sum_k \left(L_k \rho L_k^\dagger - \frac{1}{2}\{L_k^\dagger L_k, \rho\}\right)$$

where $L_k$ are Lindblad operators representing coupling to environmental baths.

### 1.4 Coherence Time Modeling
The framework computes coherence metrics:
- **$T_1$ (population decay)**: Energy relaxation time
- **$T_2$ (dephasing)**: Phase coherence time
- **Quantum Fisher information**: For metrological applications
- **Entanglement entropy**: For multi-partite systems

---

## 2. Functional Requirements

### FR-1: Hamiltonian Construction
- **FR-1.1**: Support arbitrary time-dependent Hamiltonians $H(t)$ via user-defined functions
- **FR-1.2**: Provide built-in templates for common driven systems:
  - Driven two-level systems (Rabi oscillations)
  - Parametrically driven cavities
  - Coupled cavity arrays with phase modulation
- **FR-1.3**: Support sparse and dense matrix representations
- **FR-1.4**: Allow Hamiltonian composition from multiple terms

### FR-2: Time Evolution
- **FR-2.1**: Implement multiple integration schemes:
  - Runge-Kutta 4th order (RK4)
  - Magnus expansion (2nd and 4th order)
  - Split-operator method for separable Hamiltonians
- **FR-2.2**: Adaptive time-stepping with error control
- **FR-2.3**: Compute Floquet quasi-energies and modes via:
  - Direct diagonalization of one-period propagator
  - Iterative Krylov subspace methods for large systems
- **FR-2.4**: Support both unitary (closed) and non-unitary (open) evolution

### FR-3: Lindblad Master Equation
- **FR-3.1**: Solve Lindblad equation for density matrices
- **FR-3.2**: Support multiple Lindblad operators with configurable rates
- **FR-3.3**: Include thermal baths with temperature-dependent rates
- **FR-3.4**: Compute steady states via null-space methods

### FR-4: Observable Calculation
- **FR-4.1**: Compute expectation values $\langle O(t) \rangle$ for arbitrary observables
- **FR-4.2**: Calculate quantum correlators $\langle A(t) B(t') \rangle$
- **FR-4.3**: Compute entanglement measures (von Neumann entropy, concurrence)
- **FR-4.4**: Calculate Floquet spectrum and quasi-energy statistics

### FR-5: Parameter Sweep and Optimization
- **FR-5.1**: Parallel parameter space exploration on GPU
- **FR-5.2**: Support grid search, random sampling, and Latin hypercube sampling
- **FR-5.3**: Gradient-based optimization for inverse design problems
- **FR-5.4**: Export parameter-observable maps in standard formats (HDF5, Parquet)

### FR-6: Visualization
- **FR-6.1**: Real-time 2D/3D visualization of:
  - Density matrix evolution (Hinton diagrams, Bloch sphere)
  - Observable time series
  - Quasi-energy spectra
  - Parameter sweep heatmaps
- **FR-6.2**: Interactive exploration of simulation results
- **FR-6.3**: Export publication-quality plots (SVG, PDF)
- **FR-6.4**: Animation export (MP4, GIF)

### FR-7: Data Management
- **FR-7.1**: Save/load simulation state checkpoints
- **FR-7.2**: Export results in HDF5 with self-describing metadata
- **FR-7.3**: Logging of all simulation parameters for reproducibility
- **FR-7.4**: Import/export configurations in TOML/YAML

---

## 3. Non-Functional Requirements

### NFR-1: Performance
- **NFR-1.1**: Leverage GPU acceleration for systems with Hilbert space dimension $> 64$
- **NFR-1.2**: Target $> 80\%$ GPU utilization for dense matrix operations
- **NFR-1.3**: Scale to system sizes of at least $10^6 \times 10^6$ for sparse representations
- **NFR-1.4**: Batch processing overhead $< 5\%$ for parameter sweeps

### NFR-2: Numerical Stability
- **NFR-2.1**: Maintain unitarity of evolution operators to $\epsilon < 10^{-10}$ for closed systems
- **NFR-2.2**: Preserve trace and positivity of density matrices for open systems
- **NFR-2.3**: Use double-precision (f64) arithmetic by default, with optional f32 mode
- **NFR-2.4**: Detect and report integration errors, NaN/Inf propagation

### NFR-3: Reproducibility
- **NFR-3.1**: Deterministic RNG seeding for all stochastic processes
- **NFR-3.2**: Version all input files and simulation configurations
- **NFR-3.3**: Log full provenance: compiler version, GPU driver, commit hash
- **NFR-3.4**: Regression test suite against analytical benchmarks

### NFR-4: Usability
- **NFR-4.1**: CLI with intuitive command structure and help text
- **NFR-4.2**: GUI with real-time parameter adjustment
- **NFR-4.3**: Comprehensive error messages with suggested fixes
- **NFR-4.4**: Interactive tutorial notebooks (Jupyter via Rust kernel or Python bindings)

### NFR-5: Extensibility
- **NFR-5.1**: Plugin architecture for custom Hamiltonians and observables
- **NFR-5.2**: Python FFI via PyO3 for scripting and integration
- **NFR-5.3**: Modular GPU backend supporting CUDA, ROCm, and WebGPU
- **NFR-5.4**: Documented internal APIs for third-party extensions

---

## 4. System Architecture

### 4.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interface Layer                      │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │     CLI      │  │  GUI (egui)  │  │  Python Bindings  │  │
│  └──────┬───────┘  └──────┬───────┘  └─────────┬─────────┘  │
└─────────┼──────────────────┼───────────────────┼────────────┘
          │                  │                   │
          └──────────────────┼───────────────────┘
                             │
┌─────────────────────────────────────────────────────────────┐
│                    Simulation Engine                         │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Scheduler: Parameter Sweeps, Job Queue, Results    │   │
│  └────────────────────────┬─────────────────────────────┘   │
│                           │                                  │
│  ┌────────────────────────┴────────────────────┐            │
│  │          Core Physics Engine                │            │
│  │  • Floquet Solver                           │            │
│  │  • Time Integrators (RK4, Magnus)           │            │
│  │  • Lindblad Master Equation                 │            │
│  │  • Observable Calculators                   │            │
│  └────────────┬─────────────────┬──────────────┘            │
│               │                 │                            │
│  ┌────────────▼──────┐  ┌───────▼──────────┐               │
│  │  CPU Kernels      │  │  GPU Kernels     │               │
│  │  (ndarray, BLAS)  │  │  (wgpu/CUDA)     │               │
│  └───────────────────┘  └──────────────────┘               │
└─────────────────────────────────────────────────────────────┘
                             │
┌─────────────────────────────────────────────────────────────┐
│                      Data Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ HDF5 Export  │  │  Checkpoints │  │  Config I/O  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Module Breakdown

#### 4.2.1 Core Physics (`src/core/`)
- **`hamiltonian.rs`**: Hamiltonian trait and implementations
- **`state.rs`**: Quantum state representations (ket, density matrix)
- **`integrator.rs`**: Time evolution algorithms
- **`floquet.rs`**: Floquet analysis and quasi-energy computation
- **`lindblad.rs`**: Open quantum system dynamics
- **`observables.rs`**: Observable trait and built-in operators

#### 4.2.2 GPU Backend (`src/gpu/`)
- **`backend.rs`**: Abstraction over wgpu/CUDA
- **`kernels.rs`**: GPU kernel dispatch
- **`memory.rs`**: Device memory management
- **`batch.rs`**: Batched parameter sweep execution

#### 4.2.3 UI Layer (`src/ui/`)
- **`cli.rs`**: Command-line interface using `clap`
- **`gui/`**: GUI implementation with `egui`
  - `app.rs`: Main application state
  - `widgets.rs`: Custom visualization widgets
  - `plots.rs`: Real-time plotting
- **`bindings/`**: Python FFI via PyO3

#### 4.2.4 Data Management (`src/data/`)
- **`config.rs`**: Configuration parsing (TOML/YAML)
- **`export.rs`**: HDF5 and Parquet export
- **`checkpoint.rs`**: Simulation state serialization

#### 4.2.5 Utilities (`src/utils/`)
- **`math.rs`**: Linear algebra utilities
- **`logger.rs`**: Structured logging
- **`error.rs`**: Error types and handling

### 4.3 Concurrency Model
- **Async runtime**: Tokio for I/O-bound operations (file loading, network if applicable)
- **Rayon**: Data parallelism for CPU-bound tasks
- **GPU streams**: Concurrent kernel execution and data transfer

---

## 5. APIs and Interfaces

### 5.1 Simulation Configuration Format (TOML)

```toml
[simulation]
name = "driven_cavity"
duration = 100.0  # microseconds
timestep = 0.01
integrator = "rk4"  # or "magnus2", "magnus4"

[system]
hilbert_dim = 10
hamiltonian = "driven_cavity"

[system.parameters]
omega_0 = 5.0  # GHz
omega_d = 5.1
Omega_d = 0.5
kappa = 0.01  # decay rate

[lindblad]
enabled = true
[[lindblad.operators]]
type = "annihilation"
rate = 0.01
temperature = 0.0  # K, for thermal bath

[observables]
list = ["number", "coherence", "entanglement_entropy"]
save_interval = 1.0  # microseconds

[gpu]
enabled = true
device = "auto"  # or "cuda:0", "rocm:0", "cpu"
batch_size = 256

[parameter_sweep]
enabled = false
parameter = "system.parameters.omega_d"
range = [4.5, 5.5]
num_points = 101
```

### 5.2 Rust API

#### 5.2.1 Building a Hamiltonian
```rust
use chronophoton::core::{Hamiltonian, ParametricHamiltonian};
use ndarray::Array2;
use num_complex::Complex64;

// Custom time-dependent Hamiltonian
struct DrivenTLS {
    omega_0: f64,
    omega_d: f64,
    amplitude: f64,
}

impl Hamiltonian for DrivenTLS {
    fn dim(&self) -> usize { 2 }

    fn compute(&self, t: f64, out: &mut Array2<Complex64>) {
        let phase = Complex64::new(0.0, self.omega_d * t).exp();
        out[[0, 1]] = self.amplitude * phase;
        out[[1, 0]] = self.amplitude * phase.conj();
        out[[0, 0]] = Complex64::new(self.omega_0 / 2.0, 0.0);
        out[[1, 1]] = Complex64::new(-self.omega_0 / 2.0, 0.0);
    }
}
```

#### 5.2.2 Running a Simulation
```rust
use chronophoton::simulation::{SimulationBuilder, IntegratorType};

let sim = SimulationBuilder::new()
    .hamiltonian(hamiltonian)
    .initial_state(initial_state)
    .duration(100.0)
    .timestep(0.01)
    .integrator(IntegratorType::RK4)
    .observable("number", number_operator)
    .gpu(true)
    .build()?;

let results = sim.run()?;
```

#### 5.2.3 Parameter Sweep
```rust
use chronophoton::sweep::{ParameterSweep, SweepStrategy};

let sweep = ParameterSweep::new()
    .parameter("omega_d", 4.5..5.5)
    .points(101)
    .strategy(SweepStrategy::Grid)
    .gpu_batch_size(256)
    .build()?;

let sweep_results = sweep.execute(&base_config)?;
sweep_results.save_hdf5("results.h5")?;
```

### 5.3 Python API

```python
import chronophoton as cp

# Load configuration
config = cp.Config.from_file("config.toml")

# Create simulation
sim = cp.Simulation(config)

# Run and retrieve results
results = sim.run()

# Access observables
time = results.time
number_expect = results.observable("number")

# Plot
import matplotlib.pyplot as plt
plt.plot(time, number_expect)
plt.show()

# Parameter sweep
sweep = cp.ParameterSweep(
    config,
    parameter="omega_d",
    values=np.linspace(4.5, 5.5, 101)
)
sweep_results = sweep.run(gpu=True, batch_size=256)
sweep_results.to_hdf5("results.h5")
```

### 5.4 Plugin Interface

Plugins extend ChronoPhoton functionality via dynamic loading:

```rust
// In plugin crate
use chronophoton::plugin::{Plugin, PluginMetadata};

#[chronophoton::plugin]
pub struct MyCustomHamiltonian;

impl Plugin for MyCustomHamiltonian {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "custom_hamiltonian".into(),
            version: "0.1.0".into(),
            description: "Custom driven system".into(),
        }
    }

    fn load(&mut self, registry: &mut Registry) {
        registry.register_hamiltonian("my_custom", |params| {
            Box::new(/* custom implementation */)
        });
    }
}
```

---

## 6. Use Cases and Examples

### 6.1 Use Case 1: Driven Two-Level System

**Objective**: Simulate Rabi oscillations in a two-level atom driven by a near-resonant laser.

**Configuration** (`examples/driven_tls.toml`):
```toml
[system]
hilbert_dim = 2
hamiltonian = "driven_tls"

[system.parameters]
omega_0 = 5.0  # Transition frequency (GHz)
omega_d = 5.0  # Drive frequency
Omega_R = 0.5  # Rabi frequency

[simulation]
duration = 50.0  # ns
timestep = 0.1
```

**Expected Results**:
- Population oscillations at Rabi frequency
- Floquet quasi-energies: $\pm \Omega_R / 2$

**Validation**: Compare with analytical solution:
$$P_e(t) = \sin^2\left(\frac{\Omega_R t}{2}\right)$$

### 6.2 Use Case 2: Parametric Cavity Time Crystal

**Objective**: Observe subharmonic frequency generation in a parametrically driven cavity.

**Configuration** (`examples/parametric_cavity.toml`):
```toml
[system]
hilbert_dim = 20  # Fock space truncation
hamiltonian = "parametric_cavity"

[system.parameters]
omega_c = 10.0  # Cavity frequency (GHz)
omega_p = 20.0  # Pump frequency (2 * omega_c)
g = 0.3        # Parametric coupling

[lindblad]
enabled = true
[[lindblad.operators]]
type = "annihilation"
rate = 0.05  # Photon loss

[simulation]
duration = 200.0
timestep = 0.05
```

**Expected Results**:
- Emergent oscillations at $\omega_c$ despite $\omega_p = 2\omega_c$ drive
- Bifurcation in steady-state photon number vs. pump strength

### 6.3 Use Case 3: Coupled Cavity Array

**Objective**: Simulate topological edge modes in a driven cavity lattice.

**System**: 10 coupled cavities with alternating coupling strengths (SSH model).

**Configuration** (`examples/ssh_cavity.toml`):
```toml
[system]
hilbert_dim = 1024  # 2^10 for 10 cavities, each dim=2 (empty/filled)
hamiltonian = "coupled_cavities"

[system.parameters]
omega_c = 5.0
J1 = 1.0  # Strong coupling
J2 = 0.5  # Weak coupling
omega_d = 5.0
modulation_amplitude = 0.2

[simulation]
duration = 100.0
timestep = 0.1
```

**Expected Results**:
- Edge-localized Floquet modes when $J1 \neq J2$
- Numerical verification of topological winding number

---

## 7. Testing Methodology

### 7.1 Unit Tests
- **Hamiltonian Construction**: Verify hermiticity, correct dimensions
- **Integrators**: Check energy conservation for closed systems
- **Observable Calculation**: Test against known operators (Pauli matrices, etc.)
- **GPU Kernels**: Compare CPU and GPU results within tolerance

### 7.2 Integration Tests
- **End-to-End Simulation**: Run full simulation pipeline from config to output
- **Checkpoint/Resume**: Verify state restoration produces identical trajectories
- **Parameter Sweep**: Test batching and result aggregation

### 7.3 Validation Tests
- **Analytical Benchmarks**:
  - Driven TLS: Compare with exact Rabi solution
  - Free oscillator: Verify coherent state evolution
  - Decay: Exponential relaxation with rate $\kappa$
- **Floquet Quasi-Energies**: Match known results for quantum kicked rotor
- **Numerical Convergence**: Demonstrate timestep scaling behavior

### 7.4 Performance Tests
- **Scaling**: Measure runtime vs. Hilbert space dimension
- **GPU Utilization**: Profile kernel execution and memory bandwidth
- **Batch Efficiency**: Compare single vs. batched parameter sweeps

### 7.5 Regression Tests
- Lock analytical test cases with reference outputs
- CI pipeline runs full suite on every commit
- Automated benchmarking to detect performance regressions

### 7.6 Validation Against Literature
- Reproduce published results:
  - Floquet topological insulators [Lindner et al., Nature Physics 2011]
  - Parametric oscillator phase diagrams [Minkov & Savona, Optica 2016]
  - Driven-dissipative phase transitions [Carusotto & Ciuti, Rev. Mod. Phys. 2013]

---

## 8. Security and Robustness

### 8.1 Memory Safety
- **Rust Guarantees**: Borrow checker prevents data races and use-after-free
- **Unsafe Code Auditing**: All `unsafe` blocks documented and minimized
- **GPU Memory**: Explicit allocation/deallocation tracking to prevent leaks

### 8.2 Panic Handling
- **Graceful Degradation**: Catch panics in worker threads, log, and continue
- **GPU Errors**: Detect device errors, fall back to CPU if possible
- **User Input**: Validate all configuration parameters before simulation start

### 8.3 Numerical Integrity
- **Bounds Checking**: Ensure array accesses within range
- **NaN/Inf Detection**: Check results after each integration step
- **Matrix Validity**: Verify density matrices remain positive semi-definite

### 8.4 Data Integrity
- **Checksum**: Hash simulation outputs for tamper detection
- **Versioning**: Embed schema version in HDF5 files
- **Atomic Writes**: Use temp files + rename for checkpoint saving

### 8.5 Dependency Security
- **Audit**: Regularly scan dependencies with `cargo audit`
- **Minimal Surface**: Limit number of external crates
- **Pinning**: Lock dependency versions for reproducible builds

---

## 9. Deployment

### 9.1 Binary Distribution

#### 9.1.1 CLI Binary
- **Platform Support**: Linux (x86_64, aarch64), Windows, macOS
- **Static Linking**: Bundle all dependencies except system libs
- **Installation**:
  ```bash
  cargo install chronophoton
  chronophoton --version
  ```

#### 9.1.2 GUI Binary
- **Packaging**: AppImage (Linux), .exe installer (Windows), .dmg (macOS)
- **GPU Detection**: Auto-detect available devices on first launch
- **Updates**: Built-in update checker (opt-in)

### 9.2 Container Images

**Dockerfile** (multi-stage build):
```dockerfile
FROM rust:1.75 AS builder
RUN apt-get update && apt-get install -y cuda-toolkit
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM nvidia/cuda:12.0-runtime
COPY --from=builder /app/target/release/chronophoton /usr/local/bin/
ENTRYPOINT ["chronophoton"]
```

**Usage**:
```bash
docker run --gpus all -v $(pwd)/config.toml:/config.toml \
  chronophoton:latest run --config /config.toml
```

### 9.3 Python Wheel
- **Build**: `maturin build --release`
- **Distribution**: Publish to PyPI as `chronophoton`
- **Platform Wheels**: Pre-built for Linux, Windows, macOS

### 9.4 Continuous Integration

`.github/workflows/ci.yml`:
- **Build**: Compile on Linux, Windows, macOS
- **Test**: Run full test suite
- **Benchmark**: Track performance over time
- **Release**: Automatic binary/wheel build on tags

### 9.5 Documentation Deployment
- **API Docs**: `cargo doc` → host on docs.rs
- **User Guide**: mdBook → GitHub Pages
- **Examples**: Jupyter notebooks in repo

---

## 10. Key Performance Indicators

### 10.1 Runtime Performance

| System Size (Hilbert Dim) | Target Runtime (GPU) | Target Runtime (CPU) |
|---------------------------|----------------------|----------------------|
| 10                        | < 1 ms/step          | < 1 ms/step          |
| 100                       | < 5 ms/step          | < 50 ms/step         |
| 1,000                     | < 50 ms/step         | < 5 s/step           |
| 10,000                    | < 500 ms/step        | N/A (GPU only)       |

### 10.2 Numerical Accuracy

| Metric                     | Threshold                  |
|----------------------------|----------------------------|
| Unitarity violation        | $\|U^\dagger U - I\| < 10^{-10}$ |
| Trace preservation         | $|\text{Tr}(\rho) - 1| < 10^{-10}$ |
| Positivity                 | All eigenvalues $\geq -10^{-12}$ |
| Energy conservation (closed)| $\Delta E / E_0 < 10^{-8}$ |

### 10.3 GPU Utilization

| Metric                     | Target                     |
|----------------------------|----------------------------|
| SM Occupancy               | > 80%                      |
| Memory Bandwidth           | > 70% of theoretical peak  |
| Kernel Launch Overhead     | < 5% of total runtime      |

### 10.4 Scalability

| Batch Size | Speedup (vs. sequential) | GPU Memory Usage |
|------------|--------------------------|------------------|
| 10         | 8x                       | < 1 GB           |
| 100        | 70x                      | < 4 GB           |
| 1,000      | 500x                     | < 16 GB          |

### 10.5 Code Quality

| Metric                     | Target                     |
|----------------------------|----------------------------|
| Test Coverage              | > 85%                      |
| Documentation Coverage     | 100% of public APIs        |
| Clippy Warnings            | 0                          |
| Unsafe Code                | < 5% of codebase           |

### 10.6 User Experience

| Metric                     | Target                     |
|----------------------------|----------------------------|
| CLI Response Time          | < 100 ms                   |
| GUI Frame Rate             | > 60 FPS                   |
| Error Message Clarity      | User testing validation    |
| Documentation Completeness | All features with examples |

---

## Appendix A: Glossary

- **Floquet Hamiltonian**: Effective time-independent Hamiltonian for periodically driven systems
- **Quasi-Energy**: Eigenvalue of Floquet Hamiltonian, analogous to energy in time-independent systems
- **Lindblad Operator**: Operator describing coupling to environment in Markovian master equation
- **Fock State**: Number eigenstate of quantum harmonic oscillator
- **Bloch Sphere**: Geometrical representation of qubit state space

## Appendix B: References

1. M. Bukov et al., "Universal high-frequency behavior of periodically driven systems: from dynamical stabilization to Floquet engineering," *Adv. Phys.* **64**, 139 (2015)
2. N.Y. Yao et al., "Discrete time crystals: rigidity, criticality, and realizations," *Phys. Rev. Lett.* **118**, 030401 (2017)
3. H.-P. Breuer & F. Petruccione, *The Theory of Open Quantum Systems* (Oxford, 2002)
4. V. Giovannetti et al., "Quantum metrology," *Phys. Rev. Lett.* **96**, 010401 (2006)

## Appendix C: Changelog

### Version 0.1.0 (Initial Release)
- Core Floquet solver
- RK4 and Magnus integrators
- Lindblad master equation
- Basic GUI with real-time plotting
- HDF5 export
- Example configurations

### Version 0.2.0 (Planned)
- Multi-GPU support
- Python bindings
- Quantum trajectory unraveling
- Topological invariant calculation

---

## Appendix D: Contributing

ChronoPhoton welcomes contributions! See `CONTRIBUTING.md` for:
- Code style guide (rustfmt + clippy)
- PR submission process
- Testing requirements
- Adding new Hamiltonians/observables
