# ChronoPhoton User Guide

## Installation

### From crates.io
```bash
cargo install chronophoton
```

### From source
```bash
git clone https://github.com/yourusername/chronophoton.git
cd chronophoton
cargo build --release
```

## Quick Start

### 1. Generate a Configuration Template
```bash
chronophoton template --output my_config.toml --template-type driven_tls
```

### 2. Edit Configuration
```toml
[simulation]
name = "my_simulation"
duration = 100.0
timestep = 0.1

[system]
hilbert_dim = 2
hamiltonian = "driven_tls"
```

### 3. Run Simulation
```bash
chronophoton run --config my_config.toml --output results.h5
```

### 4. Launch GUI
```bash
chronophoton gui --config my_config.toml
```

## Configuration Reference

See example configurations in `examples/configs/`:
- `driven_tls.toml`: Two-level system
- `parametric_cavity.toml`: Driven cavity
- `ssh_cavity.toml`: Coupled cavity array

## Using the Rust API

```rust
use chronophoton::prelude::*;

let hamiltonian = DrivenTLS::new(5.0, 5.0, 0.5);
let sim = SimulationBuilder::new()
    .hamiltonian(hamiltonian)
    .initial_state(QuantumState::ground_state(2))
    .duration(50.0)
    .timestep(0.1)
    .build()?;

let results = sim.run()?;
```

## Parameter Sweeps

Enable in configuration:
```toml
[parameter_sweep]
enabled = true
parameter = "system.parameters.omega_d"
range = [4.5, 5.5]
num_points = 101
```

Or via API:
```rust
let sweep = ParameterSweep::new()
    .parameter("omega_d", 4.5..5.5)
    .points(101)
    .gpu_batch_size(256)
    .build()?;

let results = sweep.execute(&config)?;
```

## GPU Acceleration

Enable GPU in configuration:
```toml
[gpu]
enabled = true
device = "cuda:0"  # or "auto", "cpu"
batch_size = 256
```

## Troubleshooting

### GPU not detected
- Check drivers are installed
- Try `device = "auto"` in config
- Fall back to `enabled = false` for CPU

### Numerical instability
- Reduce timestep
- Use higher precision
- Check Hamiltonian hermiticity

### Performance issues
- Enable GPU for dim > 64
- Increase batch_size for sweeps
- Use sparse representations if available

## Examples

See `examples/` directory for complete examples.
