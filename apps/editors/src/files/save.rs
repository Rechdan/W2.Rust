use std::{fs::File, io::Write, path::PathBuf};

use crate::consts::{SERVER_LIST_FILE, SERVER_NAME_FILE, STRDEF_FILE};

pub fn save(file: PathBuf, buf: Vec<u8>) -> bool {
    match File::create(file) {
        Ok(mut file) => match file.write_all(&buf) {
            Ok(_result) => true,
            Err(_error) => false,
        },
        Err(_error) => false,
    }
}

pub fn server_list(folder: PathBuf, buf: Vec<u8>) -> bool {
    save(folder.join(SERVER_LIST_FILE), buf)
}

pub fn server_name(folder: PathBuf, buf: Vec<u8>) -> bool {
    save(folder.join(SERVER_NAME_FILE), buf)
}

pub fn strdef(folder: PathBuf, buf: Vec<u8>) -> bool {
    save(folder.join(STRDEF_FILE), buf)
}
