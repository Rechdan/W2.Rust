use fixedstr::zstr;
use std::{mem::transmute, path::PathBuf};

use crate::{
    consts::{STRDEF_MESSAGES_LEN, STRDEF_SIZE},
    encodings::utf8_to_windows1252,
    files::{load, save},
};

#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct Strdef {
    pub messages: [zstr<128>; STRDEF_MESSAGES_LEN],
    pub unk: u32,
}

impl Strdef {
    pub fn new(folder: PathBuf) -> Option<Self> {
        match load::strdef(folder.clone()) {
            Some(buf) => match TryInto::<[u8; STRDEF_SIZE]>::try_into(buf) {
                Ok(buf) => Some(unsafe { transmute(buf) }),
                Err(_error) => None,
            },
            None => None,
        }
    }

    pub fn save(&mut self, folder: PathBuf, messages: Vec<String>) {
        for i in 0..STRDEF_MESSAGES_LEN {
            let message = messages[i].clone();
            let buf = utf8_to_windows1252(&message);
            self.messages[i] = zstr::<128>::from_raw(&buf);
        }
        let buf: [u8; STRDEF_SIZE] = unsafe { transmute(*self) };
        save::strdef(folder, buf.to_vec());
    }
}
