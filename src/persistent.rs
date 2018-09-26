pub fn get(key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    let len = unsafe { ::sys::_get_len(key.as_ptr(), key.len()) };
    let mut val = Vec::with_capacity(len);
    unsafe { val.set_len(len) };
    unsafe { ::sys::_get(key.as_ptr(), key.len(), val.as_mut_ptr()) };
    val
}

pub fn set(key: &str, val: &[u8]) {
    let key = key.as_bytes();
    unsafe { ::sys::_set(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}
