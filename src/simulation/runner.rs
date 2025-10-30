use crate::core::{integrator, Hamiltonian, IntegratorType, Observable, QuantumState};
use crate::simulation::SimulationResults;
use crate::utils::Result;

pub struct SimulationRunner {
    hamiltonian: Box<dyn Hamiltonian>,
    initial_state: QuantumState,
    duration: f64,
    timestep: f64,
    integrator: Box<dyn crate::core::Integrator>,
    observables: Vec<(String, Box<dyn Observable>)>,
    gpu_enabled: bool,
}

impl SimulationRunner {
    pub fn new(
        hamiltonian: Box<dyn Hamiltonian>,
        initial_state: QuantumState,
        duration: f64,
        timestep: f64,
        integrator_type: IntegratorType,
        observables: Vec<(String, Box<dyn Observable>)>,
        gpu_enabled: bool,
    ) -> Result<Self> {
        let integrator = integrator::create_integrator(integrator_type);

        Ok(Self {
            hamiltonian,
            initial_state,
            duration,
            timestep,
            integrator,
            observables,
            gpu_enabled,
        })
    }

    pub fn run(&self) -> Result<SimulationResults> {
        tracing::info!("Starting simulation");

        let num_steps = (self.duration / self.timestep).ceil() as usize;
        let mut state = self.initial_state.clone();
        let mut results = SimulationResults::new();

        for step in 0..num_steps {
            let t = step as f64 * self.timestep;

            for (name, observable) in &self.observables {
                let value = observable.expectation_pure(&state);
                results.add_observable(name, t, value);
            }

            self.integrator
                .step(self.hamiltonian.as_ref(), &mut state, t, self.timestep)?;

            if step % 100 == 0 {
                tracing::debug!("Step {}/{}", step, num_steps);
            }
        }

        tracing::info!("Simulation complete");
        Ok(results)
    }
}
