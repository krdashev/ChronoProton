#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub min: f64,
    pub max: f64,
}

impl Parameter {
    pub fn new(name: impl Into<String>, min: f64, max: f64) -> Self {
        Self {
            name: name.into(),
            min,
            max,
        }
    }

    pub fn linspace(&self, n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| self.min + (self.max - self.min) * i as f64 / (n - 1) as f64)
            .collect()
    }
}
