use fixedstr::zstr;
use std::{mem::transmute, panic::catch_unwind, path::PathBuf};

use crate::{
    consts::{SERVER_LIST_SIZE, SERVER_NAME_SIZE},
    enc_dec,
    get_file::{get_server_list_file, get_server_name_file},
};

#[repr(C, packed(4))]
pub struct ServerList {
    pub key: u32,
    pub worlds: [(zstr<64>, [zstr<64>; 10]); 10],
}

impl ServerList {
    pub fn new(folder: PathBuf) -> Option<Self> {
        match get_server_list_file(folder.clone()) {
            Some((size, buf)) => match size {
                SERVER_LIST_SIZE => match catch_unwind(|| {
                    let mut buf = TryInto::<[u8; SERVER_LIST_SIZE]>::try_into(buf).unwrap();
                    let buf_encoded = buf.get_mut(4..).unwrap();
                    enc_dec::server_list(buf_encoded);
                    unsafe { transmute(buf) }
                }) {
                    Ok(result) => Some(result),
                    Err(_error) => None,
                },
                _ => None,
            },
            None => None,
        }
    }
}

#[repr(C, packed)]
pub struct ServerName {
    pub newbies: zstr<9>,
    pub worlds: [zstr<9>; 10],
    pub newbiews_count: u32,
    pub counts: [u32; 10],
}

impl ServerName {
    pub fn new(folder: PathBuf) -> Option<Self> {
        match get_server_name_file(folder.clone()) {
            Some((size, buf)) => match size {
                SERVER_NAME_SIZE => match catch_unwind(|| {
                    let mut buf = TryInto::<[u8; SERVER_NAME_SIZE]>::try_into(buf).unwrap();
                    buf[0] = ((buf[0] as i32) - 100) as u8;
                    unsafe { transmute(buf) }
                }) {
                    Ok(result) => Some(result),
                    Err(_error) => None,
                },
                _ => None,
            },
            None => None,
        }
    }
}
