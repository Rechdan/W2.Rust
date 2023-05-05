use std::{mem::size_of, ptr::read, slice::from_raw_parts};

pub fn serialize<T: Sized>(s: &T) -> Vec<u8> {
    unsafe { from_raw_parts((s as *const T) as *const u8, size_of::<T>()).to_vec() }
}

pub fn deserialize<T: Sized>(buf: &[u8]) -> Option<T> {
    if size_of::<T>() != buf.len() {
        ()
    }

    Some(unsafe { read(buf.as_ptr() as *const T) })
}
