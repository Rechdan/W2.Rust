use fixedstr::zstr;
use std::{mem::transmute, path::PathBuf};

use crate::{
    consts::SERVER_LIST_SIZE,
    files::{load, save},
    security::{decoders, encoders},
};

#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct ServerList {
    pub key: u32,
    pub worlds: [(zstr<64>, [zstr<64>; 10]); 10],
}

impl ServerList {
    pub fn new(folder: PathBuf) -> Option<Self> {
        match load::server_list(folder.clone()) {
            Some(buf) => match TryInto::<[u8; SERVER_LIST_SIZE]>::try_into(buf) {
                Ok(mut buf) => {
                    decoders::server_list(buf.get_mut(4..).unwrap());
                    Some(unsafe { transmute(buf) })
                }
                Err(_error) => None,
            },
            None => None,
        }
    }

    pub fn save(&self, folder: PathBuf) {
        let mut buf: [u8; SERVER_LIST_SIZE] = unsafe { transmute(*self) };
        encoders::server_list(buf.get_mut(4..).unwrap());
        save::server_list(folder, buf.to_vec());
    }
}
