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
    PausedLogging,
    Closed,
}

impl Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::WaitingLocal => write!(f, "Aguardando local"),
            ConnectionState::WaitingRemote => write!(f, "Aguardando remoto"),
            ConnectionState::Connection => write!(f, "Aguardando conexão"),
            ConnectionState::Connected => write!(f, "Conectado"),
            ConnectionState::PausedLogging => {
                write!(f, "Conectado, pausado")
            }
            ConnectionState::Closed => write!(f, "Fechado"),
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

    pub fn close(&self) {
        let mut state = self.state.blocking_lock();
        match state.clone() {
            ConnectionState::Closed => {}
            _ => *state = ConnectionState::Closed,
        }
    }

    // private helpers

    fn begin_connection(&mut self) {
        let mut conn = self.clone();

        RT.spawn(async move {
            conn.accept_connection().await;
            match RT.spawn_blocking(move || conn.close()).await {
                Ok(_) => {}
                Err(_) => {}
            }
        });
    }

    async fn add_packet(&mut self, direction: PacketDirection, buf: &mut Vec<u8>) {
        loop {
            match buf.get(0..2) {
                Some(sizer_buf) => match TryInto::<[u8; 2]>::try_into(sizer_buf) {
                    Ok(sizer_buf) => {
                        let size = u16::from_le_bytes(sizer_buf) as usize;

                        if buf.len() < size {
                            break;
                        }

                        let state = self.state.lock().await.clone();

                        match state {
                            ConnectionState::PausedLogging | ConnectionState::Closed => {}

                            _ => {
                                match buf.get_mut(0..size) {
                                    Some(packet_buf) => {
                                        decode(packet_buf);

                                        let packet = Packet::new_async(
                                            direction.clone(),
                                            packet_buf.to_vec(),
                                        )
                                        .await;

                                        let mut packets = self.packets.lock().await;
                                        packets.push(packet);
                                    }

                                    None => break,
                                };
                            }
                        };

                        match buf.get(size..) {
                            Some(new_buf) => {
                                *buf = new_buf.to_vec();
                            }
                            None => {
                                buf.clear();
                            }
                        }
                    }

                    Err(_) => break,
                },

                None => break,
            };
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
                c.append(format!("ID: {}", &conn.id), |rt| rt.monospace())
                    .append(format!("Estado: {}", state), |rt| rt.monospace())
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

    pub fn render_actions(&self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            self.render_pause_resume_action(ui);
            self.render_close_action(ui);
            self.render_remove_action(ui);
        });
    }

    fn render_pause_resume_action(&self, ui: &mut Ui) {
        let mut state = self.state.blocking_lock();
        match state.clone() {
            ConnectionState::Connected => {
                if ui.button("Pausar logs").clicked() {
                    *state = ConnectionState::PausedLogging;
                }
            }
            ConnectionState::PausedLogging => {
                if ui.button("Resumir logs").clicked() {
                    *state = ConnectionState::Connected;
                }
            }
            _ => {}
        }
    }

    fn render_close_action(&self, ui: &mut Ui) {
        let state = self.state.blocking_lock().clone();
        match state {
            ConnectionState::Closed => {}
            _ => {
                if ui.button("Fechar").clicked() {
                    self.close();
                }
            }
        }
    }

    fn render_remove_action(&self, ui: &mut Ui) {
        if ui.button("Remover").clicked() {
            STATE.remove_connection(self.clone());
        }
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
            Ok(listener) => loop {
                let state = self.state.lock().await.clone();

                match state {
                    ConnectionState::Closed => break,
                    _ => {}
                };

                let timer = timeout(Duration::from_secs_f32(0.1), listener.accept()).await;

                match timer {
                    Ok(accept_result) => match accept_result {
                        Ok((stream, _addr)) => {
                            let conn = self.clone();

                            let (mut reader, mut writer) = (
                                conn.local_reader.lock().await,
                                conn.local_writer.lock().await,
                            );

                            let (reader2, writer2) = split(stream);

                            *reader = Some(reader2);
                            *writer = Some(writer2);

                            *self.state.lock().await = ConnectionState::WaitingRemote;

                            break;
                        }
                        Err(_) => self.close(),
                    },
                    Err(_) => {}
                };
            },
            Err(_) => self.close(),
        };

        let state = self.state.lock().await.clone();

        match state {
            ConnectionState::WaitingRemote => self.connect_to_remote().await,
            _ => {}
        }
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

        *self.state.lock().await = ConnectionState::Connection;

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
                    let state = self.state.lock().await.clone();

                    match state {
                        ConnectionState::Closed => break,
                        _ => {}
                    };

                    let mut buf = [0u8; 1024];

                    let timer = timeout(Duration::from_secs_f32(0.1), reader.read(&mut buf)).await;

                    match timer {
                        Ok(size_result) => match size_result {
                            Ok(size) => {
                                if size < 1 {
                                    break;
                                } else {
                                    let mut buf = buf.to_vec();
                                    buf.truncate(size);

                                    let mut buf_to_persisted = buf.clone();
                                    persisted_buf.append(&mut buf_to_persisted);

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

                                    let state = self.state.lock().await.clone();

                                    match state {
                                        ConnectionState::Connection => {
                                            {
                                                *self.state.lock().await =
                                                    ConnectionState::Connected;
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

                                        ConnectionState::Connected
                                        | ConnectionState::PausedLogging => {
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

                            _ => break,
                        },

                        Err(_) => {}
                    };
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
