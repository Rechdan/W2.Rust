use std::{fs::File, io::Read, path::PathBuf};

use crate::consts::{SERVER_LIST_FILE, SERVER_LIST_SIZE, SERVER_NAME_FILE, SERVER_NAME_SIZE};

type FileReturn<const T: usize> = Option<(usize, [u8; T])>;

pub fn get_file<const T: usize>(file: PathBuf) -> FileReturn<T> {
    if file.exists() {
        match File::open(file) {
            Ok(mut file) => {
                let buf = &mut [0; T];

                match file.read(buf) {
                    Ok(size) => {
                        return Some((size, buf.clone()));
                    }

                    Err(_error) => {}
                }
            }

            Err(_error) => {}
        }
    }

    None
}

pub fn get_server_list_file(folder: PathBuf) -> FileReturn<SERVER_LIST_SIZE> {
    get_file(folder.join(SERVER_LIST_FILE))
}

pub fn get_server_name_file(folder: PathBuf) -> FileReturn<SERVER_NAME_SIZE> {
    get_file(folder.join(SERVER_NAME_FILE))
}
