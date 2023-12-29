use fixedstr::zstr;
use std::{mem::transmute, panic::catch_unwind};

use crate::{consts::SERVER_LIST_SIZE, decoders};

#[repr(C, packed(4))]
pub struct ServerList {
    pub key: u32,
    pub worlds: [(zstr<64>, [zstr<64>; 10]); 10],
}

impl ServerList {
    pub fn new(buf: &[u8]) -> Option<Self> {
        match catch_unwind(|| {
            let mut buf = TryInto::<[u8; SERVER_LIST_SIZE]>::try_into(buf).unwrap();
            let buf_encoded = buf.get_mut(4..).unwrap();
            decoders::server_list(buf_encoded);
            unsafe { transmute(buf) }
        }) {
            Ok(result) => Some(result),
            Err(_error) => None,
        }
    }
}
