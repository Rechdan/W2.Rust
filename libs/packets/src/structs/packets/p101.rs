use crate::{
    strings::{bytes_to_str, str_to_bytes},
    structs::header::SHeader,
};

#[repr(C)]
pub struct P101 {
    pub header: SHeader,
    message: [u8; 80],
    unk1: [u8; 48],
}

impl P101 {
    pub fn new(message: &str) -> P101 {
        let mut p = P101 {
            header: SHeader::new_packet::<P101>(0x101),
            message: [0; 80],
            unk1: [0; 48],
        };

        p.set_message(message);

        p
    }

    pub fn get_message(&self) -> String {
        bytes_to_str(&self.message)
    }
    pub fn set_message(&mut self, message: &str) {
        str_to_bytes(&mut self.message, message)
    }
}
