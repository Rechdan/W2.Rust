use std::{fs::File, io::Read, path::PathBuf};

use crate::consts::{SERVER_LIST_FILE, SERVER_NAME_FILE, STRDEF_FILE};

pub fn load(file: PathBuf) -> Option<Vec<u8>> {
    match File::open(file) {
        Ok(mut file) => {
            let buf = &mut Vec::new();

            match file.read_to_end(buf) {
                Ok(_size) => Some(buf.clone()),
                Err(_error) => None,
            }
        }
        Err(_error) => None,
    }
}

pub fn server_list(folder: PathBuf) -> Option<Vec<u8>> {
    load(folder.join(SERVER_LIST_FILE))
}

pub fn server_name(folder: PathBuf) -> Option<Vec<u8>> {
    load(folder.join(SERVER_NAME_FILE))
}

pub fn strdef(folder: PathBuf) -> Option<Vec<u8>> {
    load(folder.join(STRDEF_FILE))
}
