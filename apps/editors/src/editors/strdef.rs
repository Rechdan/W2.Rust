use egui::{Layout, ScrollArea, TextEdit, Ui};
use egui_extras::{Size, StripBuilder};
use std::path::PathBuf;

use crate::{
    consts::STRDEF_MESSAGES_LEN,
    encodings::windows1252_to_utf8,
    structs::strdef::Strdef,
};

use super::EditorRender;

pub struct StrDefEditor {
    folder: PathBuf,
    strdef: Strdef,
    messages: Vec<String>,
}

impl EditorRender for StrDefEditor {
    fn name() -> &'static str {
        "Strdef"
    }

    fn new(folder: PathBuf) -> Option<Box<Self>> {
        let strdef = match Strdef::new(folder.clone()) {
            Some(strdef) => strdef,
            None => return None,
        };

        let mut messages = Vec::new();

        for i in 0..STRDEF_MESSAGES_LEN {
            let msg = strdef.messages[i];
            let msg = msg.chars();
            let msg = msg.as_str();
            let msg = msg.as_bytes();
            messages.push(windows1252_to_utf8(msg));
        }

        Some(Box::new(Self {
            folder,
            strdef,
            messages,
        }))
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui.set_width(360.0);
        ui.set_height(400.0);

        StripBuilder::new(ui)
            .size(Size::remainder())
            .size(Size::exact(24.0))
            .vertical(|mut s| {
                s.cell(|ui| self.fields(ui));
                s.cell(|ui| self.footer_actions(ui));
            });
    }
}

impl StrDefEditor {
    fn fields(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show_rows(ui, 20.0, STRDEF_MESSAGES_LEN, |ui, row_range| {
            for row in row_range {
                ui.add_sized(
                    [ui.available_width(), 20.0],
                    TextEdit::singleline(&mut self.messages[row]),
                );
            }
        });
    }

    fn footer_actions(&mut self, ui: &mut Ui) {
        ui.allocate_ui_with_layout(
            ui.available_size(),
            Layout::bottom_up(egui::Align::Min),
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Salvar").clicked() {
                        self.strdef.save(self.folder.clone(), self.messages.clone());
                    }
                });
            },
        );
    }
}
