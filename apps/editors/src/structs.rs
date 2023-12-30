use fixedstr::zstr;
use std::{mem::transmute, panic::catch_unwind, path::PathBuf};

use crate::{
    consts::{SERVER_LIST_SIZE, SERVER_NAME_SIZE},
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
            Some(buf) => match catch_unwind(|| {
                let mut buf = TryInto::<[u8; SERVER_LIST_SIZE]>::try_into(buf).unwrap();
                decoders::server_list(buf.get_mut(4..).unwrap());
                unsafe { transmute(buf) }
            }) {
                Ok(result) => Some(result),
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

#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ServerName {
    pub worlds: [zstr<9>; 11],
    pub counts: [[u8; 4]; 11],
}

impl ServerName {
    pub fn new(folder: PathBuf) -> Option<Self> {
        match load::server_name(folder.clone()) {
            Some(buf) => match catch_unwind(|| {
                let mut buf = TryInto::<[u8; SERVER_NAME_SIZE]>::try_into(buf).unwrap();
                buf[0] = ((buf[0] as i32) - 100) as u8;
                unsafe { transmute(buf) }
            }) {
                Ok(result) => Some(result),
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
