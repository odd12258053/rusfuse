use std::mem::size_of;
use std::slice;

#[allow(dead_code)]
pub fn to_bytes<T>(data: &T) -> &[u8] {
    let v = data as *const T as *const u8;
    let s = size_of::<T>();
    return unsafe { slice::from_raw_parts(v, s) };
}
