use chronophoton::{
    data::config::Config, simulation::SimulationBuilder, ui::gui::App, utils::logger,
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
    Run {
        #[arg(short, long)]
        config: PathBuf,

        #[arg(long)]
        gpu: Option<bool>,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    Gui {
        config: Option<PathBuf>,
    },

    Validate {
        config: PathBuf,
    },

    Template {
        #[arg(short, long, default_value = "config.toml")]
        output: PathBuf,

        #[arg(short, long, default_value = "driven_tls")]
        template_type: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init()?;

    let args = Args::parse();

    match args.command {
        Commands::Run {
            config,
            gpu,
            output,
        } => {
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
                Box::new(|_cc| Ok(Box::new(app))),
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

        Commands::Template {
            output,
            template_type,
        } => {
            tracing::info!("Generating template: {}", template_type);
            let template = Config::generate_template(&template_type)?;
            template.save(&output)?;
            println!("Template saved to {:?}", output);
        }
    }

    Ok(())
}
