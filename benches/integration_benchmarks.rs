
use chronophoton::core::systems::DrivenTLS;
use chronophoton::core::{QuantumState, IntegratorType};
use chronophoton::simulation::SimulationBuilder;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_driven_tls(c: &mut Criterion) {
    c.bench_function("driven_tls_100_steps", |b| {
        b.iter(|| {
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

            black_box(sim.run().unwrap())
        })
    });
}

criterion_group!(benches, benchmark_driven_tls);
criterion_main!(benches);
