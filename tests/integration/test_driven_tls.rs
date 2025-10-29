//! Integration tests for driven two-level system

#[cfg(test)]
mod tests {
    use chronophoton::core::systems::DrivenTLS;
    use chronophoton::core::{QuantumState, IntegratorType};
    use chronophoton::simulation::SimulationBuilder;
    use approx::assert_relative_eq;

    #[test]
    fn test_driven_tls_simulation() {
        let hamiltonian = DrivenTLS::new(5.0, 5.0, 0.5);
        let initial_state = QuantumState::ground_state(2);

        let sim = SimulationBuilder::new()
            .hamiltonian(hamiltonian)
            .initial_state(initial_state)
            .duration(10.0)
            .timestep(0.1)
            .integrator(IntegratorType::RK4)
            .build()
            .unwrap();

        let results = sim.run();
        assert!(results.is_ok());
    }
}
