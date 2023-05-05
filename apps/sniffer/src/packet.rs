use egui::{vec2, Color32, Response, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use packets::{serializer::deserialize, structs::header::SHeader};
use std::{fmt::Display, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    consts::SPACE,
    functions::{
        bordered_container::{bordered_container, BodyContainerSettings},
        bytes_cell::byte_cell,
        monospaced::monospaced,
        selectable_item::selectable_item,
    },
    statics::{LAST_ID, STATE},
};

#[derive(Debug, Clone)]
pub enum PacketDirection {
    SND,
    RCV,
}

impl Display for PacketDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketDirection::SND => write!(f, "SND"),
            PacketDirection::RCV => write!(f, "RCV"),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        *self.id == *other.id
    }
    fn ne(&self, other: &Self) -> bool {
        *self.id != *other.id
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: Arc<u64>,
    direction: Arc<PacketDirection>,
    buffer: Arc<Vec<u8>>,
    header: Arc<SHeader>,
    columns_count: Arc<Mutex<usize>>,
    selected_buffer_index: Arc<Mutex<usize>>,
}

impl Packet {
    // initialization

    fn new(id: u64, direction: PacketDirection, buffer: Vec<u8>) -> Self {
        Self {
            id: Arc::new(id),
            direction: Arc::new(direction),
            buffer: Arc::new(buffer.clone()),
            header: Arc::new(deserialize::<SHeader>(&buffer[0..12]).unwrap()),
            columns_count: Arc::new(Mutex::new(0usize)),
            selected_buffer_index: Arc::new(Mutex::new(0usize)),
        }
    }

    pub async fn new_async(direction: PacketDirection, buffer: Vec<u8>) -> Self {
        Self::new(LAST_ID.async_next_packet_id().await, direction, buffer)
    }

    pub fn new_sync(direction: PacketDirection, buffer: Vec<u8>) -> Self {
        Self::new(LAST_ID.sync_next_packet_id(), direction, buffer)
    }

    // helpers

    pub fn get_buffer(&self) -> Arc<Vec<u8>> {
        self.buffer.clone()
    }

    fn update_columns_count(&self, new_columns_count: usize) {
        let mut columns_count = self.columns_count.blocking_lock();

        if columns_count.clone() != new_columns_count {
            *columns_count = new_columns_count;
        }
    }

    // ui

    pub fn render_list(&self, ui: &mut Ui, selected: bool) -> Response {
        let response = selectable_item(
            ui,
            selected,
            |s| s.set_padding(vec2(SPACE, SPACE / 2.0)),
            |c| {
                c.append(
                    format!("[{}]: 0x{:04X}", &self.direction, &self.header.packet_id),
                    |rt| {
                        rt.monospace().color(match selected {
                            true => Color32::WHITE,
                            false => match &*self.direction {
                                PacketDirection::SND => Color32::LIGHT_GREEN,
                                PacketDirection::RCV => Color32::LIGHT_RED,
                            },
                        })
                    },
                )
            },
        );

        response
    }

    pub fn render_buffer_table(&self, ui: &mut Ui) {
        let index_column_width = 85.0;

        let buffer = STATE.get_buffer_view();
        let buffer_len = buffer.len();

        let available_width = ui.available_width() - index_column_width;
        let columns_count = (&available_width / 40.0) as usize;
        let mut total_rows = buffer_len / columns_count;

        if (buffer_len % columns_count) > 0 {
            total_rows += 1;
        }

        let columns_width = ((&available_width - (SPACE * 2.0)) / (columns_count as f32)) - SPACE;

        let aheight = ui.available_height();

        let selected_index = self.selected_buffer_index.blocking_lock().clone();

        self.update_columns_count(columns_count);

        TableBuilder::new(ui)
            .striped(true)
            .auto_shrink([false, true])
            .max_scroll_height(aheight)
            .column(Column::exact(index_column_width))
            .columns(Column::exact(columns_width), columns_count)
            .header(20.0, |mut header| {
                header.col(|_| {});

                for i in 0..columns_count {
                    header.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(RichText::from(format!("|{}|", i)).monospace());
                        });
                    });
                }
            })
            .body(|body| {
                body.rows(20.0, total_rows, |i, mut row| {
                    row.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(
                                RichText::from(format!(
                                    "{:04} to {:04}",
                                    i * columns_count,
                                    (((i + 1) * columns_count) - 1).min(buffer_len - 1)
                                ))
                                .monospace(),
                            );
                        });
                    });

                    for j in 0..columns_count {
                        let buffer_index = (i * columns_count) + j;

                        row.col(|ui| {
                            match buffer.get(buffer_index) {
                                Some(value) => {
                                    let cell = byte_cell(
                                        ui,
                                        buffer_index == selected_index,
                                        value.to_string(),
                                    );

                                    if cell.clicked() {
                                        *self.selected_buffer_index.blocking_lock() = buffer_index;
                                    }
                                }
                                _ => (),
                            };
                        });
                    }
                });
            });
    }

    pub fn render_info(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.render_header_info(ui);
            self.render_selected_byte_info(ui);
        });
    }

    fn render_header_info(&self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s.set_fill_height(false),
            |ui| {
                ui.label(monospaced("Header information:".to_string()));

                bordered_container(
                    ui,
                    |s| s.set_fill_height(false),
                    |ui| {
                        match *self.header {
                            SHeader {
                                size,
                                key,
                                checksum,
                                packet_id,
                                client_id,
                                timestamp,
                            } => {
                                ui.label(monospaced(format!("     Size: {:05}", size)));
                                ui.label(monospaced(format!("      Key: {:03}", key)));
                                ui.label(monospaced(format!(" CheckSum: {:03}", checksum)));
                                ui.label(monospaced(format!("Packet ID: 0x{:04X}", packet_id)));
                                ui.label(monospaced(format!("Client ID: {:05}", client_id)));
                                ui.label(monospaced(format!("TimeStamp: {:010}", timestamp)));
                            }
                        };
                    },
                );
            },
        );
    }

    fn render_selected_byte_info(&self, ui: &mut Ui) {
        bordered_container(
            ui,
            |s| s.set_fill_height(false),
            |ui| {
                let selected_index = self.selected_buffer_index.blocking_lock().clone();

                ui.label(monospaced(format!("Selected index: {}", selected_index)));

                bordered_container(
                    ui,
                    |s: BodyContainerSettings| s.set_fill_height(false),
                    |ui| {
                        match self.buffer.get(selected_index..) {
                            Some(arr) => {
                                let arr = arr.to_vec();

                                match arr.get(0..1) {
                                    Some(arr) => match TryInto::<[u8; 1]>::try_into(arr) {
                                        Ok(arr) => {
                                            ui.label(monospaced(format!(
                                                " sByte: {}\n  Byte: {}",
                                                i8::from_le_bytes(arr),
                                                u8::from_le_bytes(arr)
                                            )));
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                };

                                match arr.get(0..2) {
                                    Some(arr) => match TryInto::<[u8; 2]>::try_into(arr) {
                                        Ok(arr) => {
                                            ui.label(monospaced(format!(
                                                " Short: {}\nuShort: {}",
                                                i16::from_le_bytes(arr),
                                                u16::from_le_bytes(arr)
                                            )));
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                };

                                match arr.get(0..4) {
                                    Some(arr) => match TryInto::<[u8; 4]>::try_into(arr) {
                                        Ok(arr) => {
                                            ui.label(monospaced(format!(
                                                "   Int: {}\n  uInt: {}",
                                                i32::from_le_bytes(arr),
                                                u32::from_le_bytes(arr)
                                            )));
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                };

                                match arr.get(0..8) {
                                    Some(arr) => match TryInto::<[u8; 8]>::try_into(arr) {
                                        Ok(arr) => {
                                            ui.label(monospaced(format!(
                                                "  Long: {}\n uLong: {}",
                                                i64::from_le_bytes(arr),
                                                u64::from_le_bytes(arr)
                                            )));
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                };
                            }
                            _ => (),
                        };
                    },
                );
            },
        );
    }
}
