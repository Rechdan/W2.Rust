use std::mem::size_of;

use crate::structs::ServerList;

pub const SERVERLIST_KEY_LEN: usize = 63;

pub const SERVERLIST_KEY: [u8; SERVERLIST_KEY_LEN] = [
    0xc1, 0xb6, 0xc0, 0xcc, 0xc0, 0xd3, 0xc6, 0xd1, 0xc6, 0xae, 0xbe, 0xcf, 0xc8, 0xa3, 0xc8, 0xad,
    0xc0, 0xdb, 0xbe, 0xf7, 0xc0, 0xbb, 0xc0, 0xa7, 0xc7, 0xd1, 0xbd, 0xba, 0xc5, 0xa9, 0xb8, 0xb3,
    0xc6, 0xae, 0xc0, 0xd4, 0xb4, 0xcf, 0xb4, 0xd9, 0xb8, 0xb8, 0xc7, 0xd1, 0xb1, 0xdb, 0xb7, 0xce,
    0xbe, 0xcf, 0xc8, 0xad, 0xc8, 0xad, 0xc7, 0xd2, 0xc1, 0xd9, 0xa4, 0xbb, 0xa4, 0xbb, 0x00,
];

pub const SERVER_LIST_SIZE: usize = size_of::<ServerList>();