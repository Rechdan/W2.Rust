use crate::consts::{SERVER_LIST_KEY, SERVER_LIST_KEY_LEN};

pub fn server_list(buf: &mut [u8]) -> bool {
    for i in 0..10 {
        for j in 0..11 {
            for k in (64 - SERVER_LIST_KEY_LEN)..64 {
                let buf_index = 704 * i + 64 * j + k;
                let key_index = 63 - k;

                let buf_value = buf.get_mut(buf_index).unwrap();
                let key_value = SERVER_LIST_KEY.get(key_index).unwrap();

                *buf_value = ((*buf_value as i16) - (*key_value as i16)) as u8;
            }
        }
    }

    return true;
}
