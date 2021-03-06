use crate::tools::remove_placeholder;
use eframe::egui::CtxRef;
use rust_fsm::StateMachine;
use std::error::Error;
use std::vec;

rust_fsm::state_machine! {
    derive(PartialEq,Debug,Clone)
    DevToolsStates(Initial)

    Initial(Initialize) => UseSDKBasic,
    UseSDKBasic(OpenFile) => QueryFileName,
    QueryFileName(ObtainedFileName) => UseSDKWithFile,
    UseSDKWithFile(PrintTableOfContents) => UseSDKWithFile,
    UseSDKWithFile(ExportInAnotherFormat) => QueryOutputFileName,
    QueryOutputFileName(ObtainedOutputFileName) => UseSDKWithFile,
}

#[derive(Clone)]
struct Transition {
    kind: DevToolsStatesInput,
    associated_string: String,
}

pub struct DevToolsApplication {
    transitions: Vec<Transition>,
    snippets: Snippets,
    states: StateMachine<DevToolsStates>,
}

impl DevToolsApplication {
    pub fn new() -> DevToolsApplication {
        let transitions = vec![Transition {
            kind: DevToolsStatesInput::Initialize,
            associated_string: String::new(),
        }];
        DevToolsApplication {
            transitions: transitions,
            snippets: Snippets::new(),
            states: StateMachine::new(),
        }
    }

    fn apply_transitions(&self, transitions: &Vec<Transition>) -> String {
        let mut resulting_string = String::new();
        for transition in transitions {
            resulting_string = match transition.kind {
                DevToolsStatesInput::Initialize => self.snippets.initial.clone(),
                DevToolsStatesInput::OpenFile => resulting_string,
                DevToolsStatesInput::ObtainedFileName => {
                    self.apply_open_file(&resulting_string, &transition.associated_string)
                }
                DevToolsStatesInput::PrintTableOfContents => {
                    self.apply_print_table_of_contents(&resulting_string)
                }
                DevToolsStatesInput::ExportInAnotherFormat => resulting_string,
                DevToolsStatesInput::ObtainedOutputFileName => self.apply_export_in_another_format(
                    &resulting_string,
                    &transition.associated_string,
                ),
            }
        }
        resulting_string
    }

    fn apply_open_file(&self, resulting_string: &String, file_name: &String) -> String {
        let resulting_string =
            str::replace(&resulting_string, "%placeholder%", &self.snippets.open_file);
        str::replace(&resulting_string, "%file_name%", file_name)
    }

    fn apply_export_in_another_format(
        &self,
        resulting_string: &String,
        output_file_name: &String,
    ) -> String {
        let resulting_string = str::replace(
            &resulting_string,
            "%placeholder%",
            &self.snippets.export_in_another_format,
        );
        str::replace(&resulting_string, "%output_file_name%", output_file_name)
    }

    fn apply_print_table_of_contents(&self, resulting_string: &String) -> String {
        str::replace(
            &resulting_string,
            "%placeholder%",
            &self.snippets.print_table_of_contents,
        )
    }

    pub fn get_file_content(&self) -> String {
        let file_content = self.apply_transitions(&self.transitions);
        remove_placeholder(&file_content)
    }

    pub fn render(&mut self, ctx: &CtxRef) {
        match self.states.state() {
            DevToolsStatesState::Initial => {
                self.states.consume(&DevToolsStatesInput::Initialize);
                ()
            }
            DevToolsStatesState::UseSDKBasic => self.render_use_SDK(&ctx),
            DevToolsStatesState::QueryFileName => self.render_query_file_name(&ctx),
            DevToolsStatesState::UseSDKWithFile => self.render_use_SDK_with_file(&ctx),
            DevToolsStatesState::QueryOutputFileName => self.render_query_output_file_name(&ctx),
        }
    }

    pub fn render_use_SDK(&mut self, ctx: &CtxRef) {
        let file_content = self.apply_transitions(&self.transitions);
        let mut preview = remove_placeholder(&file_content);

        eframe::egui::CentralPanel::default().show(&ctx, |ui| {
            ui.text_edit_multiline(&mut preview);
        });

        eframe::egui::SidePanel::right("DevToolsSidePanel").show(&ctx, |ui| {
            if ui.button("Open file").clicked() {
                let default_file =
                    String::from(r#"C:\Users\victor.trejo\hackathon2022_test_files\file_1.t16"#);
                self.transitions.push(Transition {
                    kind: DevToolsStatesInput::ObtainedFileName,
                    associated_string: default_file,
                });
                self.states.consume(&DevToolsStatesInput::OpenFile);
            }
        });
    }

    pub fn render_use_SDK_with_file(&mut self, ctx: &CtxRef) {
        let file_content = self.apply_transitions(&self.transitions);
        let mut preview = remove_placeholder(&file_content);

        eframe::egui::CentralPanel::default().show(&ctx, |ui| {
            ui.text_edit_multiline(&mut preview);
        });

        eframe::egui::SidePanel::right("DevToolsSidePanel").show(&ctx, |ui| {
            if ui.button("Print table of contents").clicked() {
                self.states
                    .consume(&DevToolsStatesInput::PrintTableOfContents);
                self.transitions.push(Transition {
                    kind: DevToolsStatesInput::PrintTableOfContents,
                    associated_string: String::new(),
                })
            }

            if ui.button("Export file in another format").clicked() {
                let default_output_file = String::from(
                    r#"C:\Users\victor.trejo\hackathon2022_test_files\output_file.op2"#,
                );
                self.transitions.push(Transition {
                    kind: DevToolsStatesInput::ObtainedOutputFileName,
                    associated_string: default_output_file,
                });
                self.states
                    .consume(&DevToolsStatesInput::ExportInAnotherFormat);
            }
        });
    }

    pub fn render_query_file_name(&mut self, ctx: &CtxRef) {
        let mut file_name = self.transitions.last().unwrap().associated_string.clone();
        eframe::egui::CentralPanel::default()
            .show(&ctx, |ui| ui.text_edit_multiline(&mut file_name));

        eframe::egui::SidePanel::right("DevToolsSidePanel").show(&ctx, |ui| {
            if ui.button("Confirm choice").clicked() {
                self.states.consume(&DevToolsStatesInput::ObtainedFileName);
            }
        });
    }

    pub fn render_query_output_file_name(&mut self, ctx: &CtxRef) {
        let mut file_name = self.transitions.last().unwrap().associated_string.clone();
        eframe::egui::CentralPanel::default()
            .show(&ctx, |ui| ui.text_edit_multiline(&mut file_name));

        eframe::egui::SidePanel::right("DevToolsSidePanel").show(&ctx, |ui| {
            if ui.button("Confirm choice").clicked() {
                self.states
                    .consume(&DevToolsStatesInput::ObtainedOutputFileName);
                println!("{:?}", self.states.state());
            }
        });
    }
}

struct Snippets {
    pub initial: String,
    pub open_file: String,
    pub print_table_of_contents: String,
    pub export_in_another_format: String,
}

impl Snippets {
    pub fn new() -> Snippets {
        Snippets {
            initial: String::from(
                r#"#include "base/base.h"
#include "vdm/vdm.h"
#include "vdm/exam/datafile.h"

int main()
{
    %placeholder%return 0;
}
"#,
            ),
            open_file: String::from(
                r#"
    char* filename = R"(%file_name%)";
    Vint libType = -1;
    datafiletype(const_cast<char*>(filename), &libType);

    vdm_DataFun* libraryFunctions = vdm_DataFunBegin();

    datafileinit(libType, libraryFunctions);

    vdm_DataFunOpen(libraryFunctions, 0, filename, libType);

    Vint ierr = vdm_DataFunError(libraryFunctions);
    if (ierr) {
        vdm_DataFunClose(libraryFunctions);
        datafileterm(libType, libraryFunctions);
        vdm_DataFunEnd(libraryFunctions);
        return 1;
    }

    vdm_LMan* libraryManager = vdm_LManBegin();
    vdm_LManSetObject(libraryManager, VDM_DATAFUN, libraryFunctions);
    %placeholder%
    vdm_DataFunClose(libraryFunctions);
    datafileterm(libType, libraryFunctions);
    vdm_DataFunEnd(libraryFunctions);
    vdm_LManEnd(libraryManager);
    "#,
            ),
            print_table_of_contents: String::from(
                r#"
    vdm_LManSetParami(libraryManager, LMAN_VERBOSE, SYS_ON);
    vdm_LManTOC(libraryManager, "*");
    %placeholder%
    "#,
            ),
            export_in_another_format: String::from(
                r#"
    char* output_filename = R"(%output_file_name%)";
    Vint output_libType = -1;
    datafiletype(const_cast<char*>(output_filename), &output_libType);

    vdm_DataFun* output_libraryFunctions = vdm_DataFunBegin();

    datafileinit(output_libType, output_libraryFunctions);

    vdm_DataFunSetStatus(output_libraryFunctions, VDM_STATUS_NEW);

    vdm_DataFunOpen(output_libraryFunctions, 0, output_filename, output_libType);

    Vint output_ierr = vdm_DataFunError(output_libraryFunctions);
    if (output_ierr) {
        vdm_DataFunClose(output_libraryFunctions);
        datafileterm(output_libType, output_libraryFunctions);
        vdm_DataFunEnd(output_libraryFunctions);
        return 1;
    }

    vis_Model* model = vis_ModelBegin ();
    vdm_LManLoadModel(libraryManager, model);

    vdm_LMan* output_libraryManager = vdm_LManBegin();
    vdm_LManSetObject(output_libraryManager, VDM_DATAFUN, output_libraryFunctions);
    vdm_LManSaveModel(output_libraryManager, model);

    %placeholder%

    vdm_DataFunClose(output_libraryFunctions);
    datafileterm(output_libType, output_libraryFunctions);
    vdm_DataFunEnd(output_libraryFunctions);
    vdm_LManEnd(output_libraryManager);
    "#,
            ),
        }
    }
}
