use eframe::App;
use egui::{
    scroll_area::ScrollBarVisibility, style::Spacing, CentralPanel, ComboBox, Frame, ScrollArea,
    Ui, Vec2,
};
use egui_extras::{Size, StripBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    consts::{GLOBAL_IPS, SPACE},
    functions::{bordered_container::bordered_container, selectable_item::selectable_item},
    state::BufferType,
    statics::STATE,
};

#[derive(Default, Clone)]
pub struct MainWindow {
    // heights
    selected_connection_actions_height: Arc<Mutex<f32>>,
    version_width: Arc<Mutex<f32>>,
    // form
    adding_connection: Arc<Mutex<bool>>,
    selected_ip: Arc<Mutex<String>>,
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
    // initialization

    pub fn new() -> Self {
        let new = Self::default();

        if cfg!(debug_assertions) {
            let _conn = STATE.add_connections(new.selected_ip.blocking_lock().clone());
        }

        new
    }

    // ui

    fn principal(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_height(ui.available_height());

            self.header(ui);
            self.body(ui);
        });
    }

    fn header(&mut self, ui: &mut Ui) {
        let mw = self.clone();
        let mut version_width = mw.version_width.blocking_lock();

        ui.horizontal(|ui| {
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(version_width.clone()))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.horizontal(|ui| {
                            ui.horizontal_centered(|ui| {
                                ui.set_width(ui.available_width());

                                ui.heading("W2.Rust Sniffer");

                                ui.add_space(SPACE);

                                self.header_action(ui);
                            });
                        });
                    });

                    strip.cell(|ui| {
                        let curr_version_width = ui
                            .label(format!("Versão {}", env!("CARGO_PKG_VERSION")))
                            .rect
                            .width();

                        if version_width.clone() != curr_version_width {
                            *version_width = curr_version_width;
                        }
                    });
                });
        });
    }

    fn header_action(&mut self, ui: &mut Ui) {
        let adding_connection = self.adding_connection.blocking_lock().clone();

        if adding_connection {
            ui.add_enabled_ui(true, |ui| {
                if ui.button("Voltar").clicked() {
                    *self.adding_connection.blocking_lock() = false;
                }
            });
        } else {
            let connections = STATE.get_connections();

            let enabled = !connections.iter().any(|c| c.is_new_connection_disabled());

            ui.add_enabled_ui(enabled, |ui| {
                if ui.button("Nova conexão").clicked() {
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
                    ui.label("Adicionando uma nova conexão");

                    let selected_ip = self.selected_ip.blocking_lock().clone();

                    ComboBox::new("ip_select", "Selecione um endereço de IP")
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
                        if ui.button("Adicionar conexão").clicked() {
                            let conn =
                                STATE.add_connections(self.selected_ip.blocking_lock().clone());

                            STATE.set_selected_connection(Some(conn.clone()));

                            *self.adding_connection.blocking_lock() = false;
                        }
                    });
                });
            },
        );
    }

    fn list_connections(&mut self, ui: &mut Ui) {
        let mw = &mut self.clone();

        bordered_container(
            ui,
            |s| s,
            |ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.set_min_height(ui.available_height());

                    ui.label("Conexões");

                    match STATE.get_selected_connection() {
                        Some(selected_connection) => {
                            let mut selected_connection_actions_height =
                                mw.selected_connection_actions_height.blocking_lock();

                            StripBuilder::new(ui)
                                .size(Size::remainder())
                                .size(Size::exact(selected_connection_actions_height.clone()))
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        self.list_connections_scroll(ui);
                                    });

                                    strip.cell(|ui| {
                                        let rect_height = Frame::default()
                                            .show(ui, |ui| {
                                                bordered_container(
                                                    ui,
                                                    |s| s.set_fill_height(false),
                                                    |ui| {
                                                        selected_connection.render_actions(ui);
                                                    },
                                                );
                                            })
                                            .response
                                            .rect
                                            .height();

                                        if selected_connection_actions_height.clone() != rect_height
                                        {
                                            *selected_connection_actions_height = rect_height;
                                        }
                                    });
                                });
                        }
                        None => {
                            StripBuilder::new(ui)
                                .size(Size::remainder())
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        self.list_connections_scroll(ui);
                                    });
                                });
                        }
                    };
                });
            },
        );
    }

    fn list_connections_scroll(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .id_source("list_connections")
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_height(ui.available_height());

                let mut connections = STATE.get_connections();

                connections.iter_mut().for_each(|conn| {
                    let selected = STATE.is_connection_selected(conn.clone());

                    let response = conn.render_list(ui, selected);

                    if response.clicked() {
                        STATE.set_selected_connection(Some(conn.clone()));
                    }
                });
            });
    }

    fn list_packets(&mut self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s,
            |ui| {
                ui.vertical(|ui| {
                    let selected_connection = STATE.get_selected_connection();

                    ui.label(format!(
                        "Pacotes{}",
                        match selected_connection.clone() {
                            Some(conn) => format!(": {}", conn.count_packets()),
                            _ => "".to_string(),
                        }
                    ));

                    ScrollArea::vertical()
                        .id_source("list_packets")
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
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
            ui.label("Tipo de exibição:");

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
                    ui.label("Informações");

                    ScrollArea::vertical()
                        .id_source("list_information")
                        .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
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
