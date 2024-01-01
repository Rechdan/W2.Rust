use fixedstr::zstr;
use std::{mem::transmute, path::PathBuf};

use crate::{
    consts::SERVER_NAME_SIZE,
    files::{load, save},
};

#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ServerName {
    pub worlds: [zstr<9>; 11],
    pub counts: [[u8; 4]; 11],
}

impl ServerName {
    pub fn new(folder: PathBuf) -> Option<Self> {
        match load::server_name(folder.clone()) {
            Some(buf) => match TryInto::<[u8; SERVER_NAME_SIZE]>::try_into(buf) {
                Ok(mut buf) => {
                    buf[0] = ((buf[0] as i32) - 100) as u8;
                    Some(unsafe { transmute(buf) })
                }
                Err(_error) => None,
            },
            None => None,
        }
    }

    pub fn save(&self, folder: PathBuf) {
        let mut buf: [u8; SERVER_NAME_SIZE] = unsafe { transmute(*self) };
        buf[0] = ((buf[0] as i32) + 100) as u8;
        save::server_name(folder, buf.to_vec());
    }
}
