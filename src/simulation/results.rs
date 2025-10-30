use crate::utils::Result;
use num_complex::Complex64;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SimulationResults {
    observables: HashMap<String, Vec<(f64, Complex64)>>,
}

impl SimulationResults {
    pub fn new() -> Self {
        Self {
            observables: HashMap::new(),
        }
    }

    pub fn add_observable(&mut self, name: &str, time: f64, value: Complex64) {
        self.observables
            .entry(name.to_string())
            .or_default()
            .push((time, value));
    }

    pub fn get_observable(&self, name: &str) -> Option<&Vec<(f64, Complex64)>> {
        self.observables.get(name)
    }

    pub fn observable_names(&self) -> Vec<&String> {
        self.observables.keys().collect()
    }

    pub fn save(&self, _path: &Path) -> Result<()> {
        Err(crate::utils::Error::NotImplemented(
            "SimulationResults::save".to_string(),
        ))
    }

    pub fn print_summary(&self) {
        println!("Simulation Results:");
        println!("  Observables: {:?}", self.observable_names());
        for (name, data) in &self.observables {
            println!("  {}: {} data points", name, data.len());
        }
    }
}

impl Default for SimulationResults {
    fn default() -> Self {
        Self::new()
    }
}
