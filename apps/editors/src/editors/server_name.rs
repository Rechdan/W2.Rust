use egui::Ui;
use fixedstr::zstr;
use std::{path::PathBuf, str::FromStr};

use crate::structs::server_name::ServerName;

use super::EditorRender;

pub struct ServerNameEditor {
    folder: PathBuf,
    server_name: ServerName,
}

impl EditorRender for ServerNameEditor {
    fn name() -> &'static str {
        "Server Name"
    }

    fn new(folder: PathBuf) -> Option<Box<Self>> {
        let server_name = match ServerName::new(folder.clone()) {
            Some(server_name) => server_name,
            None => return None,
        };

        Some(Box::new(Self {
            folder,
            server_name,
        }))
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.set_width(240.0);
        self.server_name_editor(ui);
        self.footer_actions_render(ui);
    }
}

impl ServerNameEditor {
    fn server_name_editor(&mut self, ui: &mut Ui) {
        let server_name = &mut self.server_name;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width() / 2.0);

                ui.label("Nomes:");

                for i in 0..10 {
                    let world_name = &mut server_name.worlds[i].clone();
                    let name = &mut world_name.to_string();

                    if ui.text_edit_singleline(name).changed() {
                        server_name.worlds[i] = zstr::from(name);
                    }
                }
            });

            ui.vertical(|ui| {
                ui.set_width(ui.available_width());

                ui.label("Ordem:");

                for i in 0..10 {
                    let world_count = &mut server_name.counts[i][0].clone();
                    let count = &mut world_count.to_string();

                    if ui.text_edit_singleline(count).changed() {
                        server_name.counts[i][0] = match FromStr::from_str(count) {
                            Ok(v) => v,
                            Err(_error) => world_count.clone(),
                        };
                    }
                }
            });
        });
    }

    fn footer_actions_render(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui.button("Salvar").clicked() {
                self.server_name.save(self.folder.clone());
            }
        });
    }
}
