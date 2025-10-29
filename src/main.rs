//! ChronoPhoton CLI and GUI entry point

use chronophoton::{
    data::config::Config,
    simulation::SimulationBuilder,
    ui::{cli::Cli, gui::App},
    utils::logger,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "chronophoton")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Run a simulation from a configuration file
    Run {
        /// Path to configuration file (TOML/YAML)
        #[arg(short, long)]
        config: PathBuf,

        /// Override GPU setting
        #[arg(long)]
        gpu: Option<bool>,

        /// Output file for results
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Launch the GUI
    Gui {
        /// Optional configuration file to load
        config: Option<PathBuf>,
    },

    /// Validate a configuration file
    Validate {
        /// Path to configuration file
        config: PathBuf,
    },

    /// Generate a template configuration file
    Template {
        /// Output path for template
        #[arg(short, long, default_value = "config.toml")]
        output: PathBuf,

        /// Template type (driven_tls, cavity, coupled_cavities)
        #[arg(short, long, default_value = "driven_tls")]
        template_type: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    logger::init()?;

    let args = Args::parse();

    match args.command {
        Commands::Run { config, gpu, output } => {
            tracing::info!("Loading configuration from {:?}", config);
            let mut cfg = Config::from_file(&config)?;

            if let Some(gpu_enabled) = gpu {
                cfg.gpu.enabled = gpu_enabled;
            }

            tracing::info!("Building simulation");
            let sim = SimulationBuilder::from_config(&cfg)?;

            tracing::info!("Running simulation");
            let results = sim.run()?;

            if let Some(output_path) = output {
                tracing::info!("Saving results to {:?}", output_path);
                results.save(&output_path)?;
            } else {
                results.print_summary();
            }

            tracing::info!("Simulation complete");
        }

        Commands::Gui { config } => {
            tracing::info!("Launching GUI");
            let app = if let Some(config_path) = config {
                let cfg = Config::from_file(&config_path)?;
                App::with_config(cfg)
            } else {
                App::new()
            };

            let native_options = eframe::NativeOptions::default();
            eframe::run_native(
                "ChronoPhoton",
                native_options,
                Box::new(|_cc| Box::new(app)),
            )
            .map_err(|e| anyhow::anyhow!("GUI error: {}", e))?;
        }

        Commands::Validate { config } => {
            tracing::info!("Validating configuration {:?}", config);
            match Config::from_file(&config) {
                Ok(cfg) => {
                    cfg.validate()?;
                    println!("✓ Configuration is valid");
                }
                Err(e) => {
                    eprintln!("✗ Configuration error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Template { output, template_type } => {
            tracing::info!("Generating template: {}", template_type);
            let template = Config::generate_template(&template_type)?;
            template.save(&output)?;
            println!("Template saved to {:?}", output);
        }
    }

    Ok(())
}
