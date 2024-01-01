use egui::{Button, Ui};
use fixedstr::zstr;
use std::{path::PathBuf, str::FromStr};

use crate::structs::server_list::ServerList;

use super::EditorRender;

pub struct ServerListEditor {
    folder: PathBuf,
    server_list: ServerList,
    selected_world_index: usize,
}

impl EditorRender for ServerListEditor {
    fn name() -> &'static str {
        "Server List"
    }

    fn new(folder: PathBuf) -> Option<Box<Self>> {
        let server_list = match ServerList::new(folder.clone()) {
            Some(server_list) => server_list,
            None => return None,
        };

        Some(Box::new(Self {
            folder,
            server_list,
            selected_world_index: 0,
        }))
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.set_width(240.0);
        self.render_key_editor(ui);
        self.render_world_selector(ui);
        self.render_world_editor(ui);
        self.footer_actions(ui);
    }
}

impl ServerListEditor {
    fn render_key_editor(&mut self, ui: &mut Ui) {
        let server_list = &mut self.server_list;

        ui.vertical(|ui| {
            ui.label("Key:");

            let key = &mut server_list.key.to_string();
            if ui.text_edit_singleline(key).changed() {
                server_list.key = match FromStr::from_str(key) {
                    Ok(v) => v,
                    Err(_error) => server_list.key.clone(),
                };
            }
        });
    }

    fn render_world_selector(&mut self, ui: &mut Ui) {
        let selected_world_index = &mut self.selected_world_index;

        ui.vertical(|ui| {
            ui.label("Mundo:");

            ui.horizontal_wrapped(|ui| {
                for i in 0..10 {
                    let selected = *selected_world_index == i;

                    if ui
                        .add(Button::new((i + 1).to_string()).selected(selected))
                        .clicked()
                    {
                        *selected_world_index = i;
                    }
                }
            });
        });
    }

    fn render_world_editor(&mut self, ui: &mut Ui) {
        let server_list = &mut self.server_list;
        let selected_world_index = self.selected_world_index;

        let (world_url, world_channels) = server_list.worlds.get_mut(selected_world_index).unwrap();

        ui.vertical(|ui| {
            let url = &mut world_url.to_string();

            ui.label("Url:");

            if ui.text_edit_singleline(url).changed() {
                *world_url = zstr::from(url);
            }
        });

        ui.label("Canais:");

        for channel in world_channels {
            let addr = &mut channel.to_string();
            if ui.text_edit_singleline(addr).changed() {
                *channel = zstr::from(addr);
            }
        }
    }

    fn footer_actions(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui.button("Salvar").clicked() {
                self.server_list.save(self.folder.clone());
            }
        });
    }
}
