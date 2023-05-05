use std::sync::Arc;

use eframe::App;
use egui::{style::Spacing, CentralPanel, ComboBox, ScrollArea, Ui, Vec2};
use egui_extras::{Size, StripBuilder};
use tokio::sync::Mutex;

use crate::{
    consts::{GLOBAL_IPS, SPACE},
    functions::{bordered_container::bordered_container, selectable_item::selectable_item},
    state::BufferType,
    statics::STATE,
};

#[derive(Clone)]
pub struct MainWindow {
    // form
    adding_connection: Arc<Mutex<bool>>,
    selected_ip: Arc<Mutex<String>>,
}

impl Default for MainWindow {
    fn default() -> Self {
        let new = Self {
            adding_connection: Arc::new(Mutex::new(false)),
            selected_ip: Arc::new(Mutex::new("".to_string())),
        };

        if cfg!(debug_assertions) {
            let _conn = STATE.add_connections(new.selected_ip.blocking_lock().clone());
        }

        new
    }
}

impl App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style: egui::Style = (*ctx.style()).clone();

        style.spacing = Spacing {
            item_spacing: Vec2::new(SPACE, SPACE),
            ..Default::default()
        };

        ctx.set_style(style);

        CentralPanel::default().show(ctx, |ui| {
            self.principal(ui);
        });
    }
}

impl MainWindow {
    // ui

    fn principal(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_height(ui.available_height());

            self.header(ui);
            self.body(ui);
        });
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.horizontal_centered(|ui| {
                ui.set_width(ui.available_width());

                ui.heading("W2.Rust Sniffer");

                ui.add_space(SPACE);

                self.header_action(ui);
            });
        });
    }

    fn header_action(&mut self, ui: &mut Ui) {
        let adding_connection = self.adding_connection.blocking_lock().clone();

        if adding_connection {
            ui.add_enabled_ui(true, |ui| {
                if ui.button("Go back").clicked() {
                    *self.adding_connection.blocking_lock() = false;
                }
            });
        } else {
            let connections = STATE.get_connections();

            let enabled = !connections.iter().any(|c| c.is_new_connection_disabled());

            ui.add_enabled_ui(enabled, |ui| {
                if ui.button("New connection").clicked() {
                    *self.adding_connection.blocking_lock() = true;
                }
            });
        }
    }

    fn body(&mut self, ui: &mut Ui) {
        let adding_connection = self.adding_connection.blocking_lock().clone();

        if adding_connection {
            self.new_connection(ui);
        } else {
            let (connections_width, packets_width) = (200.0, 200.0);

            StripBuilder::new(ui)
                .size(Size::exact(connections_width))
                .size(Size::exact(packets_width))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.cell(|ui| self.list_connections(ui));
                    strip.cell(|ui| self.list_packets(ui));
                    strip.cell(|ui| {
                        ui.set_width(ui.available_width() - SPACE);
                        StripBuilder::new(ui)
                            .size(Size::relative(0.75))
                            .size(Size::relative(0.25))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| self.list_bytes(ui));
                                strip.cell(|ui| self.list_information(ui));
                            });
                    });
                });
        }
    }

    fn new_connection(&mut self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s,
            |ui| {
                ui.vertical(|ui| {
                    ui.label("Adding new connection");

                    let selected_ip = self.selected_ip.blocking_lock().clone();

                    ComboBox::new("ip_select", "Select an IP")
                        .selected_text(selected_ip.as_str())
                        .show_ui(ui, |ui| {
                            ui.set_width(200.0);

                            GLOBAL_IPS.iter().for_each(|ip| {
                                let mut current_value = selected_ip.clone();

                                if ui
                                    .selectable_value(&mut current_value, ip.to_string(), *ip)
                                    .clicked()
                                {
                                    *self.selected_ip.blocking_lock() = current_value;
                                }
                            });
                        });

                    ui.add_enabled_ui(selected_ip != "", |ui| {
                        if ui.button("Add connection").clicked() {
                            let conn =
                                STATE.add_connections(self.selected_ip.blocking_lock().clone());

                            STATE.set_selected_connection(conn.clone());

                            *self.adding_connection.blocking_lock() = false;
                        }
                    });
                });
            },
        );
    }

    fn list_connections(&mut self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s,
            |ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(ui.available_height());

                    ui.label("Connections");

                    ScrollArea::vertical()
                        .id_source("list_connections")
                        .always_show_scroll(true)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            ui.set_min_height(ui.available_height());

                            let mut connections = STATE.get_connections();

                            connections.iter_mut().for_each(|conn| {
                                let selected = STATE.is_connection_selected(conn.clone());

                                let response = conn.render_list(ui, selected);

                                if response.clicked() {
                                    STATE.set_selected_connection(conn.clone());
                                }
                            });
                        });
                });
            },
        );
    }

    fn list_packets(&mut self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s,
            |ui| {
                ui.vertical(|ui| {
                    let selected_connection = STATE.get_selected_connection();

                    ui.label(format!(
                        "Packets{}",
                        match selected_connection.clone() {
                            Some(conn) => format!(": {}", conn.count_packets()),
                            _ => "".to_string(),
                        }
                    ));

                    ScrollArea::vertical()
                        .id_source("list_packets")
                        .always_show_scroll(true)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            ui.set_min_height(ui.available_height());

                            match selected_connection.clone() {
                                Some(conn) => conn.render_packets(ui),
                                _ => (),
                            }
                        });
                });
            },
        );
    }

    fn render_buffer_option(&self, ui: &mut Ui, buffer_view_type: BufferType, text: String) {
        let selected_buffer_view_type = STATE.get_buffer_view_type();

        let button_padding = ui.style().spacing.button_padding;

        let item = selectable_item(
            ui,
            selected_buffer_view_type == buffer_view_type,
            |s| s.set_fill_width(false).set_padding(button_padding),
            |c| c.append(text, |rt| rt),
        );

        if item.clicked() {
            STATE.set_buffer_view_type(buffer_view_type);
        }
    }

    fn render_buffer_options(&self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.label("View type:");

            self.render_buffer_option(ui, BufferType::Byte, "Byte".to_string());
            self.render_buffer_option(ui, BufferType::Hex, "Hex".to_string());
            self.render_buffer_option(ui, BufferType::ASCII, "ASCII".to_string());
        });
    }

    fn list_bytes(&mut self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s,
            |ui| {
                ui.vertical(|ui| {
                    ui.label("Bytes");

                    self.render_buffer_options(ui);

                    match STATE.get_selected_connection() {
                        Some(conn) => conn.render_buffer_table(ui),
                        _ => (),
                    };
                });
            },
        );
    }

    fn list_information(&mut self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s,
            |ui| {
                ui.vertical(|ui| {
                    ui.label("Info");

                    ScrollArea::vertical()
                        .id_source("list_information")
                        .always_show_scroll(true)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            ui.set_min_height(ui.available_height());

                            match STATE.get_selected_connection() {
                                Some(conn) => conn.render_buffer_information(ui),
                                _ => (),
                            };
                        });
                });
            },
        );
    }
}
