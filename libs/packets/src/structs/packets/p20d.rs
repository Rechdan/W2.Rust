use crate::{strings::bytes_to_str, structs::header::SHeader};

#[repr(C)]
pub struct P20D {
    pub header: SHeader,
    password: [u8; 12],
    username: [u8; 12],
    unk1: [u8; 56],
    pub what1: u32,
    pub what2: u32,
    pub mac_id: [u8; 16],
}

impl P20D {
    pub fn get_username(&self) -> String {
        bytes_to_str(&self.username)
    }

    pub fn get_password(&self) -> String {
        bytes_to_str(&self.password)
    }
}
