use eframe::{egui, epi};

pub struct TrainingWheelsApplication {}

impl TrainingWheelsApplication {
    pub fn new() -> TrainingWheelsApplication {
        TrainingWheelsApplication {}
    }
}

impl eframe::epi::App for TrainingWheelsApplication {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Training Wheels");
        });
    }

    fn name(&self) -> &str {
        "Training Wheels"
    }
}
