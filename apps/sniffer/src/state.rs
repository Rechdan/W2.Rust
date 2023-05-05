use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{connection::Connection, packet::Packet};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Byte,
    Hex,
    ASCII,
}

pub struct State {
    // connections
    connections: Arc<Mutex<Box<Vec<Connection>>>>,
    selected_connection: Arc<Mutex<Option<Connection>>>,
    // packets
    selected_packet: Arc<Mutex<Option<Packet>>>,
    // buffers
    buffer_view: Arc<Mutex<Box<Vec<String>>>>,
    buffer_view_type: Arc<Mutex<BufferType>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            connections: Arc::new(Mutex::new(Box::new(Vec::new()))),
            selected_connection: Arc::new(Mutex::new(None)),

            selected_packet: Arc::new(Mutex::new(None)),

            buffer_view: Arc::new(Mutex::new(Box::new(Vec::new()))),
            buffer_view_type: Arc::new(Mutex::new(BufferType::Byte)),
        }
    }
}

impl State {
    // connections

    pub fn get_connections(&self) -> Box<Vec<Connection>> {
        self.connections.blocking_lock().clone()
    }

    pub fn add_connections(&self, ip: String) -> Connection {
        let conn = Connection::new(ip);

        let mut connections = self.connections.blocking_lock();
        connections.push(conn.clone());

        conn.clone()
    }

    // selected connection

    pub fn get_selected_connection(&self) -> Option<Connection> {
        self.selected_connection.blocking_lock().clone()
    }

    pub fn set_selected_connection(&self, conn: Connection) {
        self.set_selected_packet(None);
        *self.selected_connection.blocking_lock() = Some(conn.clone());
    }

    pub fn is_connection_selected(&self, conn: Connection) -> bool {
        let selected_connection = self.selected_connection.blocking_lock().clone();

        match selected_connection {
            Some(selected_connection) => selected_connection == conn,
            _ => false,
        }
    }

    // selected packet

    pub fn get_selected_packet(&self) -> Option<Packet> {
        self.selected_packet.blocking_lock().clone()
    }

    pub fn set_selected_packet(&self, selected_packet: Option<Packet>) {
        *self.selected_packet.blocking_lock() = selected_packet;
        self.update_buffer_view();
    }

    // buffer view

    pub fn get_buffer_view(&self) -> Box<Vec<String>> {
        self.buffer_view.blocking_lock().clone()
    }

    fn update_buffer_view(&self) {
        match self.get_selected_packet() {
            Some(packet) => {
                let buffer = packet
                    .get_buffer()
                    .iter()
                    .map(|b| match self.get_buffer_view_type() {
                        BufferType::Byte => b.to_string(),
                        BufferType::Hex => format!("{:02X}", b),
                        BufferType::ASCII => (b.clone() as char).to_string(),
                    })
                    .collect::<Vec<String>>();

                *self.buffer_view.blocking_lock() = Box::new(buffer);
            }
            _ => (),
        };

        // *self.buffer_view.blocking_lock() = buffer_view;
    }

    // buffer view type

    pub fn get_buffer_view_type(&self) -> BufferType {
        self.buffer_view_type.blocking_lock().clone()
    }

    pub fn set_buffer_view_type(&self, buffer_view_type: BufferType) {
        *self.buffer_view_type.blocking_lock() = buffer_view_type;
        self.update_buffer_view();
    }
}
