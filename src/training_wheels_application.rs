use eframe::egui::{CtxRef, FontDefinitions, FontFamily};
use eframe::{egui, epi};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::Index;

pub struct TrainingWheelsApplication {
    name_of_output_file: String,
}

impl TrainingWheelsApplication {
    pub fn new(name_of_output_file: &str) -> TrainingWheelsApplication {
        TrainingWheelsApplication {
            name_of_output_file: name_of_output_file.to_string(),
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_definition = FontDefinitions::default();
        let font_size = 26.;
        for text_style in eframe::egui::TextStyle::all() {
            font_definition
                .family_and_size
                .insert(text_style, (FontFamily::Monospace, font_size));
        }
        ctx.set_fonts(font_definition);
    }

    fn generate_file(&self, name_of_output_file: &str) -> Result<(), Box<dyn Error>> {
        let mut output_file = File::create(name_of_output_file)?;
        output_file.write("Writing in file.".as_bytes())?;
        Ok(())
    }
}

impl eframe::epi::App for TrainingWheelsApplication {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Generate file and exit").clicked() {
                self.generate_file(&self.name_of_output_file);
                frame.quit();
            }
        });
    }

    fn name(&self) -> &str {
        "Training Wheels"
    }
}
