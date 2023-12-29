use crate::consts::{SERVERLIST_KEY, SERVERLIST_KEY_LEN};

pub fn server_list(buf: &mut [u8]) -> bool {
    for i in 0..10 {
        for j in 0..11 {
            for k in (64 - SERVERLIST_KEY_LEN)..64 {
                let buf_index = 704 * i + 64 * j + k;
                let key_index = 63 - k;

                match buf.get_mut(buf_index) {
                    Some(buf_value) => match SERVERLIST_KEY.get(key_index) {
                        Some(key_value) => {
                            *buf_value = ((*buf_value as i16) - (*key_value as i16)) as u8;
                        }

                        None => {
                            println!("Server list invalid key index: {}", key_index);
                            return false;
                        }
                    },

                    None => {
                        println!("Server list invalid buf index: {}", buf_index);
                        return false;
                    }
                }
            }
        }
    }

    return true;
}
