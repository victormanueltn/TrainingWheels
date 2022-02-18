use crate::tools::remove_placeholder;
use eframe::egui::CtxRef;
use std::vec;

rust_fsm::state_machine! {
    derive(Debug)
    DevToolsStates(Initial)

    Initial(Initialize) => UseSDK,
}

pub struct DevToolsApplication {
    transitions: Vec<DevToolsStatesInput>,
    snippets: Snippets,
}

impl DevToolsApplication {
    pub fn new() -> DevToolsApplication {
        let transitions = vec![DevToolsStatesInput::Initialize];
        DevToolsApplication {
            transitions: transitions,
            snippets: Snippets::new(),
        }
    }

    fn apply_transitions(&self, transitions: &Vec<DevToolsStatesInput>) -> String {
        let mut resulting_string = String::new();
        for transition in transitions {
            resulting_string = match transition {
                DevToolsStatesInput::Initialize => self.snippets.initial.clone(),
            };
        }
        resulting_string
    }

    pub fn get_file_content(&self) -> String {
        let file_content = self.apply_transitions(&self.transitions);
        remove_placeholder(&file_content)
    }

    pub fn render(&self, ctx: &CtxRef) {
        let file_content = self.apply_transitions(&self.transitions);
        let mut preview = remove_placeholder(&file_content);
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut preview);
        });
    }
}

struct Snippets {
    pub initial: String,
}

impl Snippets {
    pub fn new() -> Snippets {
        Snippets {
            initial: r#"#include "base/base.h"
#include "vdm/vdm.h"

int main()
{
    %placeholder%return 0;
}
"#
            .to_string(),
        }
    }
}
