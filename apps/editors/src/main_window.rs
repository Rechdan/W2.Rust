use eframe::{App, Frame};
use egui::{CentralPanel, Context};

use crate::editors::Editors;

pub struct MainWindow {
    editors: Editors,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            editors: Default::default(),
        }
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("W2.Rust Editors");

                self.editors.render(ui);

                // match &mut self.state {
                //     State::None => {
                //         if ui.button("Selecionar pasta do cliente").clicked() {
                //             match FileDialog::new().pick_folder() {
                //                 Some(folder) => {
                //                     self.state = State::ClientSelected(ClientSelected {
                //                         path: folder,
                //                         editor_selected: EditorSelected::None,
                //                     });
                //                 }
                //                 None => {}
                //             };
                //         }
                //     }

                //     State::ClientSelected(client_selected) => {
                //         client_selected.render(ui);
                //     }
                // }
            });
        });
    }
}
