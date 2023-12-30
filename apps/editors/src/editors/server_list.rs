use egui::{vec2, Align, Button, Layout, Ui};
use egui_extras::{Size, StripBuilder};
use fixedstr::zstr;
use std::{
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::structs::{ServerList, ServerName};

#[derive(Clone)]
pub struct ServerListEditor {
    folder: Arc<PathBuf>,
    server_list: Arc<Mutex<ServerList>>,
    server_name: Arc<Mutex<ServerName>>,
    selected_world_index: Arc<Mutex<usize>>,
    buttons_height: Arc<Mutex<f32>>,
}

impl ServerListEditor {
    pub fn new(folder: PathBuf) -> Option<Self> {
        let server_list = match ServerList::new(folder.clone()) {
            Some(server_list) => server_list,
            None => return None,
        };

        let server_name = match ServerName::new(folder.clone()) {
            Some(server_name) => server_name,
            None => return None,
        };

        Some(Self {
            folder: Arc::new(folder),
            server_list: Arc::new(Mutex::new(server_list)),
            server_name: Arc::new(Mutex::new(server_name)),
            selected_world_index: Default::default(),
            buttons_height: Default::default(),
        })
    }

    pub fn render(&self, ui: &mut Ui) {
        StripBuilder::new(ui)
            .size(Size::relative(0.5))
            .size(Size::remainder())
            .horizontal(|mut s| {
                s.cell(|ui| {
                    ui.label("Server List");
                    ui.separator();
                    self.render_key_editor(ui);
                    self.render_world_selector(ui);
                    self.render_world_editor(ui);
                    self.footer_actions_render(ui);
                });

                s.cell(|ui| {
                    ui.label("Server Name (SN)");
                    ui.separator();
                    self.server_name_editor(ui);
                });
            });
    }

    fn render_key_editor(&self, ui: &mut Ui) {
        let mut server_list = self.server_list.lock().unwrap();

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

    fn render_world_selector(&self, ui: &mut Ui) {
        let mut selected_world_index = self.selected_world_index.lock().unwrap();

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

    fn render_world_editor(&self, ui: &mut Ui) {
        let mut server_list = self.server_list.lock().unwrap();
        let selected_world_index = self.selected_world_index.lock().unwrap().clone();

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

    fn server_name_editor(&self, ui: &mut Ui) {
        let mut server_name = self.server_name.lock().unwrap();

        StripBuilder::new(ui)
            .size(Size::relative(0.5))
            .size(Size::remainder())
            .horizontal(|mut s| {
                s.cell(|ui| {
                    for i in 0..10 {
                        let world_name = &mut server_name.worlds[i].clone();

                        ui.horizontal(|ui| {
                            let name = &mut world_name.to_string();
                            if ui.text_edit_singleline(name).changed() {
                                server_name.worlds[i] = zstr::from(name);
                            }
                        });
                    }
                });

                s.cell(|ui| {
                    for i in 0..10 {
                        let world_count = &mut server_name.counts[i][0].clone();

                        ui.horizontal(|ui| {
                            let count = &mut world_count.to_string();
                            if ui.text_edit_singleline(count).changed() {
                                server_name.counts[i][0] = match FromStr::from_str(count) {
                                    Ok(v) => v,
                                    Err(_error) => world_count.clone(),
                                };
                            }
                        });
                    }
                });
            });
    }

    fn footer_actions_render(&self, ui: &mut Ui) {
        let folder = (*self.folder).clone();
        let server_list = self.server_list.lock().unwrap();
        let server_name = self.server_name.lock().unwrap();
        let mut buttons_height = self.buttons_height.lock().unwrap();

        let min_size = vec2(
            ui.available_width(),
            ui.available_height().max(*buttons_height),
        );

        ui.allocate_ui_with_layout(min_size.clone(), Layout::bottom_up(Align::Min), |ui| {
            ui.set_min_size(min_size.clone());

            *buttons_height = ui
                .horizontal_wrapped(|ui| {
                    if ui.button("Salvar").clicked() {
                        server_list.save(folder.clone());
                        server_name.save(folder.clone());
                    }
                })
                .response
                .rect
                .height();
        });
    }
}
