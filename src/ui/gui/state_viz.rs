//! Quantum state visualization (Bloch sphere, density matrix, etc.)

/// Bloch sphere visualization
pub struct BlochSphereWidget;

impl BlochSphereWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BlochSphereWidget {
    fn default() -> Self {
        Self::new()
    }
}

/// Density matrix Hinton diagram
pub struct HintonWidget;

impl HintonWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HintonWidget {
    fn default() -> Self {
        Self::new()
    }
}
