use egui::{Response, Ui};
use enc_dec::decode;
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    join,
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::timeout,
};

use crate::{
    functions::selectable_item::selectable_item,
    packet::{Packet, PacketDirection},
    statics::{LAST_ID, RT, STATE},
};

// state of the connection

#[derive(Clone, Copy, PartialEq)]
pub enum ConnectionState {
    WaitingLocal,
    WaitingRemote,
    Connection,
    Connected,
    ConnectedNotStoringPackets,
    Closed,
}

impl Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::WaitingLocal => write!(f, "Waiting Local"),
            ConnectionState::WaitingRemote => write!(f, "Waiting Remote"),
            ConnectionState::Connection => write!(f, "Connection"),
            ConnectionState::Connected => write!(f, "Connected"),
            ConnectionState::ConnectedNotStoringPackets => {
                write!(f, "Connected, not storing packets")
            }
            ConnectionState::Closed => write!(f, "Closed"),
        }
    }
}

// source of the communication

enum DataSource {
    Local,
    Remote,
}

// connection

#[derive(Clone)]
pub struct Connection {
    pub id: Arc<u64>,
    pub ip: Arc<String>,
    state: Arc<Mutex<ConnectionState>>,
    local_reader: Arc<Mutex<Option<ReadHalf<TcpStream>>>>,
    local_writer: Arc<Mutex<Option<WriteHalf<TcpStream>>>>,
    remote_reader: Arc<Mutex<Option<ReadHalf<TcpStream>>>>,
    remote_writer: Arc<Mutex<Option<WriteHalf<TcpStream>>>>,
    packets: Arc<Mutex<Box<Vec<Packet>>>>,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        *self.id == *other.id
    }
    fn ne(&self, other: &Self) -> bool {
        *self.id != *other.id
    }
}

impl Connection {
    // initialization

    pub fn new(ip: String) -> Self {
        let mut conn = Self {
            id: Arc::new(LAST_ID.sync_next_connection_id()),
            ip: Arc::new(ip),
            state: Arc::new(Mutex::new(ConnectionState::WaitingLocal)),
            local_reader: Arc::new(Mutex::new(None)),
            local_writer: Arc::new(Mutex::new(None)),
            remote_reader: Arc::new(Mutex::new(None)),
            remote_writer: Arc::new(Mutex::new(None)),
            packets: Arc::new(Mutex::new(Box::new(Vec::new()))),
        };

        if cfg!(debug_assertions) && *conn.id == 1 {
            let mut packets = conn.packets.blocking_lock();

            packets.push(Packet::new_sync(PacketDirection::SND, vec![0u8; 256]));
            packets.push(Packet::new_sync(PacketDirection::RCV, vec![0u8; 1024]));

            *conn.state.blocking_lock() = ConnectionState::Closed;
        } else {
            conn.begin_connection();
        }

        conn
    }

    // public helpers

    pub fn is_new_connection_disabled(&self) -> bool {
        let state = self.state.blocking_lock().clone();

        match state {
            ConnectionState::WaitingLocal | ConnectionState::WaitingRemote => true,
            _ => false,
        }
    }

    pub fn count_packets(&self) -> usize {
        self.packets.blocking_lock().clone().len()
    }

    // private helpers

    fn begin_connection(&mut self) {
        let mut conn = self.clone();

        RT.spawn(async move {
            conn.accept_connection().await;
            *conn.state.lock().await = ConnectionState::Closed;
        });
    }

    async fn add_packet(&mut self, direction: PacketDirection, buf: &mut Vec<u8>) {
        loop {
            let size = u16::from_le_bytes([buf[0], buf[1]]) as usize;

            if buf.len() < size {
                break;
            } else {
                let packet_buf = &mut buf.clone()[0..size];

                decode(packet_buf);

                let packet = Packet::new_async(direction.clone(), packet_buf.to_vec()).await;

                let mut packets = self.packets.lock().await;
                packets.push(packet);

                *buf = buf.clone()[size..].to_vec();

                if buf.len() < 2 {
                    break;
                }
            }
        }
    }

    // ui

    pub fn render_list(&self, ui: &mut Ui, selected: bool) -> Response {
        let conn = self.clone();

        let state = *conn.state.blocking_lock();

        let response = selectable_item(
            ui,
            selected,
            |s| s,
            |c| {
                c.append(format!("Conn: {}", &conn.id), |rt| rt.monospace())
                    .append(format!("State: {}", state), |rt| rt.monospace())
            },
        );

        response
    }

    pub fn render_packets(&self, ui: &mut Ui) {
        let packets = self.packets.blocking_lock().clone();

        let selected_packet = STATE.get_selected_packet();

        packets.iter().for_each(|packet| {
            let selected = match selected_packet.clone() {
                Some(selected_packet) => selected_packet == packet.clone(),
                _ => false,
            };

            let response = packet.render_list(ui, selected);

            if response.clicked() {
                STATE.set_selected_packet(Some(packet.clone()));
            }
        });
    }

    pub fn render_actions(&self, _ui: &mut Ui) {
        let state = self.state.blocking_lock().clone();

        match state {
            ConnectionState::Connected | ConnectionState::ConnectedNotStoringPackets => {}
            _ => (),
        };
    }

    pub fn render_buffer_table(&self, ui: &mut Ui) {
        match STATE.get_selected_packet() {
            Some(packet) => packet.render_buffer_table(ui),
            _ => (),
        };
    }

    pub fn render_buffer_information(&self, ui: &mut Ui) {
        match STATE.get_selected_packet() {
            Some(packet) => packet.render_info(ui),
            _ => (),
        };
    }

    // tcp asyncs

    async fn accept_connection(&mut self) {
        let listener = TcpListener::bind("0.0.0.0:8281").await;

        match listener {
            Ok(listener) => {
                let stream = listener.accept().await;

                match stream {
                    Ok((stream, _addr)) => {
                        let conn = self.clone();

                        let (mut reader, mut writer) = (
                            conn.local_reader.lock().await,
                            conn.local_writer.lock().await,
                        );

                        let (reader2, writer2) = split(stream);

                        *reader = Some(reader2);
                        *writer = Some(writer2);
                    }
                    _ => {
                        return;
                    }
                }
            }
            _ => {
                return;
            }
        }

        {
            *self.state.lock().await = ConnectionState::WaitingRemote;
        }

        self.connect_to_remote().await;
    }

    async fn connect_to_remote(&mut self) {
        let timed_out = timeout(
            Duration::from_secs_f32(1.5),
            TcpStream::connect(format!("{}:8281", *self.ip)),
        )
        .await;

        match timed_out {
            Ok(result) => match result {
                Ok(stream) => {
                    let conn = self.clone();

                    let (mut reader, mut writer) = (
                        conn.remote_reader.lock().await,
                        conn.remote_writer.lock().await,
                    );

                    let (reader2, writer2) = split(stream);

                    *reader = Some(reader2);
                    *writer = Some(writer2);
                }
                _ => return,
            },
            _ => return,
        }

        {
            *self.state.lock().await = ConnectionState::Connection;
        }

        let (mut conn1, mut conn2) = (self.clone(), self.clone());

        join!(
            conn1.receive_data(DataSource::Local),
            conn2.receive_data(DataSource::Remote)
        );
    }

    async fn receive_data(&mut self, source: DataSource) {
        let mut reader = match source {
            DataSource::Local => self.local_reader.lock().await,
            DataSource::Remote => self.remote_reader.lock().await,
        };

        match &mut *reader {
            Some(reader) => {
                let mut persisted_buf: Vec<u8> = Vec::new();

                loop {
                    let mut buf = [0u8; 1024];
                    let size = reader.read(&mut buf).await;

                    match size {
                        Ok(size) => {
                            if size < 1 {
                                break;
                            } else {
                                let mut buf = buf.to_vec();
                                buf.truncate(size);

                                let mut buf_to_persisted = buf.clone();
                                persisted_buf.append(&mut buf_to_persisted);

                                {
                                    let mut writer = match source {
                                        DataSource::Local => self.remote_writer.lock().await,
                                        DataSource::Remote => self.local_writer.lock().await,
                                    };

                                    match &mut *writer {
                                        Some(writer) => {
                                            let _ = writer.write(&buf).await;
                                        }
                                        _ => {
                                            break;
                                        }
                                    }
                                }

                                let state = self.state.lock().await.clone();

                                match state {
                                    ConnectionState::Connection => {
                                        {
                                            *self.state.lock().await = ConnectionState::Connected;
                                        }

                                        match size {
                                            120 => {
                                                persisted_buf = persisted_buf[4..].to_vec();

                                                self.clone()
                                                    .add_packet(
                                                        PacketDirection::SND,
                                                        &mut persisted_buf,
                                                    )
                                                    .await;
                                            }
                                            _ => (),
                                        };
                                    }
                                    ConnectionState::Connected => {
                                        match source {
                                            DataSource::Local => {
                                                self.clone()
                                                    .add_packet(
                                                        PacketDirection::SND,
                                                        &mut persisted_buf,
                                                    )
                                                    .await
                                            }
                                            DataSource::Remote => {
                                                self.clone()
                                                    .add_packet(
                                                        PacketDirection::RCV,
                                                        &mut persisted_buf,
                                                    )
                                                    .await
                                            }
                                        };
                                    }
                                    _ => (),
                                };
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => (),
        };

        let mut writer = match source {
            DataSource::Local => self.remote_writer.lock().await,
            DataSource::Remote => self.local_writer.lock().await,
        };

        match &mut *writer {
            Some(writer) => {
                let _ = writer.shutdown().await;
            }
            _ => (),
        }
    }
}
