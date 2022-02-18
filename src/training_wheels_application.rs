use crate::DevTools::DevToolsApplication;
use eframe::egui::{CtxRef, FontDefinitions, FontFamily};
use eframe::{egui, epi};
use rust_fsm::{self, StateMachine};
use std::error::Error;
use std::fs::File;
use std::io::Write;

rust_fsm::state_machine! {
    derive(Debug)
    States(Initial)

    Initial(ChooseSDK) => UseSDK,
    Initial(InvalidChoice) => Unimplemented,
    Unimplemented(ChooseSDK) => UseSDK,
    UseSDK(GenerateFile) => Final,
}

enum SDK {
    Unknown,
    Devtools,
    CeetronDesktopComponents,
    CeetronCloudComponents,
    HoopsExchange,
    HoopsCommunicator,
    HoopsVisualize,
    HoopsPublish,
}

pub struct TrainingWheelsApplication {
    name_of_output_file: String,
    file_preview: String,
    SDK: DevToolsApplication,
    states: StateMachine<States>,
}

impl TrainingWheelsApplication {
    pub fn new(name_of_output_file: &str) -> TrainingWheelsApplication {
        TrainingWheelsApplication {
            name_of_output_file: name_of_output_file.to_string(),
            file_preview: String::new(),
            SDK: DevToolsApplication::new(),
            states: StateMachine::new(),
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

    fn generate_file(
        &self,
        name_of_output_file: &str,
        file_content: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut output_file = File::create(name_of_output_file)?;
        output_file.write(file_content.as_bytes())?;
        Ok(())
    }

    pub fn render_state(&mut self, ctx: &CtxRef) {
        match self.states.state() {
            StatesState::Initial => self.render_initial(ctx),
            StatesState::Unimplemented => self.render_unimplemented(ctx),
            StatesState::UseSDK => self.render_use_SDK(ctx),
            StatesState::Final => self.render_final(ctx),
        }
    }

    fn render_initial(&mut self, ctx: &CtxRef) {
        eframe::egui::SidePanel::left("Welcome to training wheels! Please choose a SDK").show(
            ctx,
            |ui| {
                ui.label("Welcome to training wheels! Please choose a SDK");
                if ui.button("DevTools").clicked() {
                    self.states.consume(&StatesInput::ChooseSDK);
                }
                if ui.button("Ceetron Desktop Components").clicked() {
                    self.states.consume(&StatesInput::InvalidChoice);
                }
                if ui.button("Ceetron Cloud Components").clicked() {
                    self.states.consume(&StatesInput::InvalidChoice);
                }
                if ui.button("Hoops Exchange").clicked() {
                    self.states.consume(&StatesInput::InvalidChoice);
                }
                if ui.button("Hoops Communicator").clicked() {
                    self.states.consume(&StatesInput::InvalidChoice);
                }
                if ui.button("Hoops Visualize").clicked() {
                    self.states.consume(&StatesInput::InvalidChoice);
                }
                if ui.button("Hoops Publish").clicked() {
                    self.states.consume(&StatesInput::InvalidChoice);
                }
            },
        );
    }

    fn render_unimplemented(&mut self, ctx: &CtxRef) {
        self.render_initial(ctx);
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            let mut message_for_unimplemented = "Not yet implemented! Please choose another one!";
            ui.text_edit_multiline(&mut message_for_unimplemented);
        });
    }

    fn render_final(&mut self, ctx: &CtxRef) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Thanks for using Training Wheels!");
        });
    }

    fn render_use_SDK(&mut self, ctx: &CtxRef) {
        eframe::egui::SidePanel::left("").show(ctx, |ui| {
            if ui.button("Generate file and exit").clicked() {
                self.states.consume(&StatesInput::GenerateFile);
                let file_content = self.SDK.get_file_content();
                self.generate_file(&self.name_of_output_file, &file_content);
            }
        });
        self.SDK.render(ctx);
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
        self.render_state(ctx);
    }

    fn name(&self) -> &str {
        "Training Wheels"
    }
}
