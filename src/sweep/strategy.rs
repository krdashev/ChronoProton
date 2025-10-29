//! Sweep strategies (grid, random, Latin hypercube, etc.)

/// Strategy for parameter space exploration
#[derive(Debug, Clone, Copy)]
pub enum SweepStrategy {
    Grid,
    Random,
    LatinHypercube,
}
