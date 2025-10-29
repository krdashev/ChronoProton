//! Core physics engine for ChronoPhoton
//!
//! This module contains the fundamental quantum mechanics implementations:
//! - Hamiltonians (time-dependent and time-independent)
//! - Quantum states (kets and density matrices)
//! - Time evolution integrators
//! - Floquet analysis
//! - Lindblad master equation
//! - Observable calculations

pub mod floquet;
pub mod hamiltonian;
pub mod integrator;
pub mod lindblad;
pub mod observables;
pub mod state;
pub mod systems;

pub use hamiltonian::Hamiltonian;
pub use state::{DensityMatrix, QuantumState};
pub use integrator::{Integrator, IntegratorType};
pub use observables::{Observable, ExpectationValue};
