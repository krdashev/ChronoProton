use crate::data::Config;
use eframe::egui;

pub struct App {
    config: Option<Config>,
}

impl App {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ChronoPhoton");
            ui.label("GPU-Accelerated Photonic Time Crystal Simulator");

            ui.separator();

            if self.config.is_some() {
                ui.label("Configuration loaded");
            } else {
                ui.label("No configuration loaded");
            }

            if ui.button("Load Configuration").clicked() {}

            if ui.button("Run Simulation").clicked() {}
        });
    }
}
