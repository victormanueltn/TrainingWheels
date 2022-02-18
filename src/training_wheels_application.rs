use eframe::{egui, epi};
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct TrainingWheelsApplication {
    name_of_output_file: String,
}

impl TrainingWheelsApplication {
    pub fn new(name_of_output_file: &str) -> TrainingWheelsApplication {
        TrainingWheelsApplication {
            name_of_output_file: name_of_output_file.to_string(),
        }
    }

    fn generate_file(&self, name_of_output_file: &str) -> Result<(), Box<dyn Error>> {
        let mut output_file = File::create(name_of_output_file)?;
        output_file.write("Writing in file.".as_bytes())?;
        Ok(())
    }
}

impl eframe::epi::App for TrainingWheelsApplication {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Training Wheels");
            if ui.button("Finish and generate file").clicked() {
                self.generate_file(&self.name_of_output_file);
            }
        });
    }

    fn name(&self) -> &str {
        "Training Wheels"
    }
}
