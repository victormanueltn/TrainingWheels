mod training_wheels_application;
use eframe::{egui, epi};
use training_wheels_application::TrainingWheelsApplication;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let application = TrainingWheelsApplication::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(application), native_options);
}
