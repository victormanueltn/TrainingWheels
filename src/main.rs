mod VdmTools;
mod training_wheels_application;
use eframe::{egui, epi};
use training_wheels_application::TrainingWheelsApplication;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let application = TrainingWheelsApplication::new("C:\\Users\\victor.trejo\\source\\vki-Devtools\\Tests\\hackathon2022\\file_created_by_Training_Wheels.cpp");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(application), native_options);
}
