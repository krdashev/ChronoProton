use crate::core::{Hamiltonian, IntegratorType, Observable, QuantumState};
use crate::data::Config;
use crate::simulation::SimulationRunner;
use crate::utils::{Error, Result};

pub struct SimulationBuilder {
    hamiltonian: Option<Box<dyn Hamiltonian>>,
    initial_state: Option<QuantumState>,
    duration: Option<f64>,
    timestep: Option<f64>,
    integrator_type: IntegratorType,
    observables: Vec<(String, Box<dyn Observable>)>,
    gpu_enabled: bool,
}

impl SimulationBuilder {
    pub fn new() -> Self {
        Self {
            hamiltonian: None,
            initial_state: None,
            duration: None,
            timestep: None,
            integrator_type: IntegratorType::RK4,
            observables: Vec::new(),
            gpu_enabled: false,
        }
    }

    pub fn hamiltonian(mut self, hamiltonian: impl Hamiltonian + 'static) -> Self {
        self.hamiltonian = Some(Box::new(hamiltonian));
        self
    }

    pub fn initial_state(mut self, state: QuantumState) -> Self {
        self.initial_state = Some(state);
        self
    }

    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn timestep(mut self, timestep: f64) -> Self {
        self.timestep = Some(timestep);
        self
    }

    pub fn integrator(mut self, integrator_type: IntegratorType) -> Self {
        self.integrator_type = integrator_type;
        self
    }

    pub fn observable(
        mut self,
        name: impl Into<String>,
        observable: impl Observable + 'static,
    ) -> Self {
        self.observables.push((name.into(), Box::new(observable)));
        self
    }

    pub fn gpu(mut self, enabled: bool) -> Self {
        self.gpu_enabled = enabled;
        self
    }

    pub fn build(self) -> Result<SimulationRunner> {
        let hamiltonian = self
            .hamiltonian
            .ok_or_else(|| Error::Config("Hamiltonian not specified".to_string()))?;

        let initial_state = self
            .initial_state
            .ok_or_else(|| Error::Config("Initial state not specified".to_string()))?;

        let duration = self
            .duration
            .ok_or_else(|| Error::Config("Duration not specified".to_string()))?;

        let timestep = self
            .timestep
            .ok_or_else(|| Error::Config("Timestep not specified".to_string()))?;

        SimulationRunner::new(
            hamiltonian,
            initial_state,
            duration,
            timestep,
            self.integrator_type,
            self.observables,
            self.gpu_enabled,
        )
    }

    pub fn from_config(_config: &Config) -> Result<SimulationRunner> {
        Err(Error::NotImplemented("from_config".to_string()))
    }
}

impl Default for SimulationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
