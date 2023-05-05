use std::mem::size_of;

#[derive(Debug)]
#[repr(C)]
pub struct SHeader {
    pub size: u16,
    pub key: u8,
    pub checksum: u8,
    pub packet_id: u16,
    pub client_id: u16,
    pub timestamp: u32,
}

impl SHeader {
    pub fn new() -> SHeader {
        SHeader {
            size: 0,
            key: 0,
            checksum: 0,
            packet_id: 0,
            client_id: 0,
            timestamp: 0,
        }
    }

    pub fn new_packet<T: Sized>(packet_id: u16) -> SHeader {
        SHeader {
            size: size_of::<T>() as u16,
            key: 0,
            checksum: 0,
            packet_id,
            client_id: 0,
            timestamp: 0,
        }
    }
}
