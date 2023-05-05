use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LastId {
    connections: Arc<Mutex<u64>>,
    packets: Arc<Mutex<u64>>,
}

impl Default for LastId {
    fn default() -> Self {
        Self {
            connections: Arc::new(Mutex::new(0)),
            packets: Arc::new(Mutex::new(0)),
        }
    }
}

impl LastId {
    // syncs

    pub fn sync_next_connection_id(&self) -> u64 {
        let mut id = self.connections.blocking_lock();
        let next_id = id.clone() + 1;
        *id = next_id;
        next_id
    }

    pub fn sync_next_packet_id(&self) -> u64 {
        let mut id = self.packets.blocking_lock();
        let next_id = id.clone() + 1;
        *id = next_id;
        next_id
    }

    // asyncs

    pub async fn async_next_connection_id(&self) -> u64 {
        let mut id = self.connections.lock().await;
        let next_id = id.clone() + 1;
        *id = next_id;
        next_id
    }

    pub async fn async_next_packet_id(&self) -> u64 {
        let mut id = self.packets.lock().await;
        let next_id = id.clone() + 1;
        *id = next_id;
        next_id
    }
}
