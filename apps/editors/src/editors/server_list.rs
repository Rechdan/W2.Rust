use egui::{Button, Ui};
use fixedstr::zstr;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::structs::ServerList;

#[derive(Clone)]
pub struct ServerListEditor {
    server_list: Arc<Mutex<ServerList>>,
    selected_world_index: Arc<Mutex<usize>>,
}

impl ServerListEditor {
    pub fn new(buf: &[u8]) -> Option<Self> {
        match ServerList::new(buf) {
            Some(server_list) => Some(Self {
                server_list: Arc::new(Mutex::new(server_list)),
                selected_world_index: Arc::new(Mutex::new(0)),
            }),
            None => None,
        }
    }

    pub fn render(&self, ui: &mut Ui) {
        let mut server_list = self.server_list.lock().unwrap();

        ui.vertical(|ui| {
            ui.label("Key:");

            let key = &mut server_list.key.to_string();
            ui.text_edit_singleline(key);
            server_list.key = match FromStr::from_str(key) {
                Ok(v) => v,
                Err(_error) => server_list.key.clone(),
            };
        });

        ui.add_space(4.0);

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

        ui.add_space(4.0);

        let (world_url, world_channels) =
            server_list.worlds.get_mut(*selected_world_index).unwrap();

        ui.vertical(|ui| {
            let url = &mut world_url.to_string();

            ui.label("Url:");

            if ui.text_edit_singleline(url).changed() {
                *world_url = zstr::from(url);
            }
        });

        ui.add_space(4.0);

        ui.label("Canais:");

        for channel in world_channels {
            let addr = &mut channel.to_string();
            if ui.text_edit_singleline(addr).changed() {
                *channel = zstr::from(addr);
            }
        }
    }
}
