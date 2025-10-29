//! ChronoPhoton: GPU-Accelerated Photonic Time Crystal Simulator
//!
//! A high-performance Rust framework for simulating periodically driven quantum systems,
//! photonic time crystals, and open quantum dynamics using GPU acceleration.
//!
//! # Overview
//!
//! ChronoPhoton provides:
//! - Floquet theory solvers for time-periodic Hamiltonians
//! - GPU-accelerated time evolution
//! - Lindblad master equation for open quantum systems
//! - Parameter sweep capabilities
//! - Real-time visualization
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use chronophoton::prelude::*;
//!
//! let hamiltonian = DrivenTLS::new(5.0, 5.0, 0.5);
//! let sim = SimulationBuilder::new()
//!     .hamiltonian(hamiltonian)
//!     .initial_state(QuantumState::ground_state(2))
//!     .duration(50.0)
//!     .timestep(0.1)
//!     .build()
//!     .unwrap();
//!
//! let results = sim.run().unwrap();
//! ```

pub mod core;
pub mod data;
pub mod gpu;
pub mod simulation;
pub mod sweep;
pub mod ui;
pub mod utils;

#[cfg(feature = "plugin")]
pub mod plugin;

#[cfg(feature = "python")]
pub mod bindings;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::{
        hamiltonian::Hamiltonian,
        state::{QuantumState, DensityMatrix},
        integrator::{Integrator, IntegratorType},
        observables::{Observable, ExpectationValue},
    };
    pub use crate::simulation::{SimulationBuilder, SimulationResults};
    pub use crate::data::config::Config;
    pub use crate::utils::error::Result;
}

// Re-export commonly used types
pub use crate::prelude::*;
