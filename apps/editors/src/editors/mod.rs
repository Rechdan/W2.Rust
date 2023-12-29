use egui::{Button, ScrollArea, Ui};
use egui_extras::{Size, StripBuilder};
use rfd::FileDialog;
use std::{fs::File, io::Read, path::PathBuf, sync::Mutex};

use self::server_list::ServerListEditor;

pub mod server_list;

#[derive(Default, Clone)]
enum EditorSelected {
    #[default]
    None,
    ServerList(ServerListEditor),
}

#[derive(Default)]
pub struct Editors {
    folder_selected: Mutex<Option<PathBuf>>,
    editor_selected: Mutex<EditorSelected>,
}

impl Editors {
    pub fn render(&self, ui: &mut Ui) {
        let folder_selected = self.folder_selected.lock().unwrap().clone();

        match folder_selected.clone() {
            Some(folder) => {
                self.render_selected_folder(ui, folder.clone());
                self.render_body(ui, folder.clone());
            }

            None => {
                if ui.button("Selecionar pasta do cliente").clicked() {
                    match FileDialog::new().pick_folder() {
                        Some(new_folder) => {
                            *self.folder_selected.lock().unwrap() = Some(new_folder);
                        }
                        None => {}
                    };
                }
            }
        };
    }

    fn render_selected_folder(&self, ui: &mut Ui, folder: PathBuf) {
        ui.horizontal(|ui| {
            ui.label("Pasta:");

            if ui.link(folder.display().to_string()).clicked() {}
        });
    }

    fn render_body(&self, ui: &mut Ui, folder: PathBuf) {
        StripBuilder::new(ui)
            .size(Size::exact(200.0))
            .size(Size::remainder())
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    ui.group(|ui| {
                        ui.set_min_size(ui.available_size());

                        self.render_editors_list(ui, folder.clone());
                    });
                });

                strip.cell(|ui| {
                    ui.group(|ui| {
                        ui.set_min_size(ui.available_size());

                        ScrollArea::vertical()
                            .id_source("editor_scroll")
                            .show(ui, |ui| {
                                match self.editor_selected.lock().unwrap().clone() {
                                    EditorSelected::None => {
                                        ui.label("Selecione um editor ao lado");
                                    }

                                    EditorSelected::ServerList(server_list) => {
                                        server_list.render(ui);
                                    }
                                };
                            });
                    });
                });
            });
    }

    fn render_editors_list(&self, ui: &mut Ui, folder: PathBuf) {
        ui.vertical_centered_justified(|ui| {
            self.render_server_list_editor_button(ui, folder);
        });
    }

    fn render_server_list_editor_button(&self, ui: &mut Ui, folder: PathBuf) {
        let path = folder.join("serverlist.bin");

        ui.add_enabled_ui(path.exists(), |ui| {
            let mut editor_selected = self.editor_selected.lock().unwrap();

            let selected = match *editor_selected {
                EditorSelected::ServerList(_) => true,
                _ => false,
            };

            if ui
                .add(Button::new("Server List").selected(selected))
                .clicked()
            {
                let buf = &mut Vec::new();

                match File::open(path).unwrap().read_to_end(buf) {
                    Ok(_size) => match ServerListEditor::new(buf) {
                        Some(server_list) => {
                            *editor_selected = EditorSelected::ServerList(server_list);
                        }

                        None => {}
                    },

                    Err(_error) => {}
                };
            }
        });
    }
}
