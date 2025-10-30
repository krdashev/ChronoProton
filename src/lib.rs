
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

pub mod prelude {
    pub use crate::core::{
        hamiltonian::Hamiltonian,
        integrator::{Integrator, IntegratorType},
        observables::{ExpectationValue, Observable},
        state::{DensityMatrix, QuantumState},
    };
    pub use crate::data::config::Config;
    pub use crate::simulation::{SimulationBuilder, SimulationResults};
    pub use crate::utils::error::Result;
}

pub use crate::prelude::*;
