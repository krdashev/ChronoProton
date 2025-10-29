//! Configuration file parsing and validation

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::utils::{Error, Result};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub simulation: SimulationConfig,
    pub system: SystemConfig,
    #[serde(default)]
    pub lindblad: LindbladConfig,
    pub observables: ObservablesConfig,
    #[serde(default)]
    pub gpu: GpuConfig,
    #[serde(default)]
    pub parameter_sweep: ParameterSweepConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub name: String,
    pub duration: f64,
    pub timestep: f64,
    #[serde(default = "default_integrator")]
    pub integrator: String,
}

fn default_integrator() -> String {
    "rk4".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub hilbert_dim: usize,
    pub hamiltonian: String,
    #[serde(default)]
    pub parameters: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LindbladConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub operators: Vec<LindbladOperatorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LindbladOperatorConfig {
    pub r#type: String,
    pub rate: f64,
    #[serde(default)]
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservablesConfig {
    pub list: Vec<String>,
    #[serde(default = "default_save_interval")]
    pub save_interval: f64,
}

fn default_save_interval() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_device")]
    pub device: String,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            device: default_device(),
            batch_size: default_batch_size(),
        }
    }
}

fn default_device() -> String {
    "auto".to_string()
}

fn default_batch_size() -> usize {
    256
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParameterSweepConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub parameter: String,
    #[serde(default)]
    pub range: Vec<f64>,
    #[serde(default)]
    pub num_points: usize,
}

impl Config {
    /// Load configuration from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;

        let config = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&contents)
                .map_err(|e| Error::Config(format!("TOML parse error: {}", e)))?
        } else {
            serde_yaml::from_str(&contents)
                .map_err(|e| Error::Config(format!("YAML parse error: {}", e)))?
        };

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::to_string_pretty(self)
                .map_err(|e| Error::Serialization(format!("TOML error: {}", e)))?
        } else {
            serde_yaml::to_string(self)
                .map_err(|e| Error::Serialization(format!("YAML error: {}", e)))?
        };

        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.simulation.duration <= 0.0 {
            return Err(Error::InvalidParameter(
                "Duration must be positive".to_string(),
            ));
        }

        if self.simulation.timestep <= 0.0 {
            return Err(Error::InvalidParameter(
                "Timestep must be positive".to_string(),
            ));
        }

        if self.system.hilbert_dim == 0 {
            return Err(Error::InvalidParameter(
                "Hilbert dimension must be positive".to_string(),
            ));
        }

        Ok(())
    }

    /// Generate a template configuration
    pub fn generate_template(template_type: &str) -> Result<Self> {
        match template_type {
            "driven_tls" => Ok(Self::driven_tls_template()),
            _ => Err(Error::InvalidParameter(format!(
                "Unknown template type: {}",
                template_type
            ))),
        }
    }

    fn driven_tls_template() -> Self {
        let mut parameters = std::collections::HashMap::new();
        parameters.insert("omega_0".to_string(), 5.0);
        parameters.insert("omega_d".to_string(), 5.0);
        parameters.insert("rabi_freq".to_string(), 0.5);

        Self {
            simulation: SimulationConfig {
                name: "driven_tls".to_string(),
                duration: 50.0,
                timestep: 0.1,
                integrator: "rk4".to_string(),
            },
            system: SystemConfig {
                hilbert_dim: 2,
                hamiltonian: "driven_tls".to_string(),
                parameters,
            },
            lindblad: LindbladConfig::default(),
            observables: ObservablesConfig {
                list: vec!["population".to_string()],
                save_interval: 1.0,
            },
            gpu: GpuConfig::default(),
            parameter_sweep: ParameterSweepConfig::default(),
        }
    }
}
