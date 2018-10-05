static mut REASON_BUFFER: [u8; 65536] = [0; 65536];

pub fn tag() -> String {
    let written = unsafe { ::sys::_reason_tag(REASON_BUFFER.as_mut_ptr()) };
    String::from_utf8(unsafe { &REASON_BUFFER[0..written] }.to_vec()).unwrap()
}

// We cannot use REASON_BUFFER from inside the callback.
pub unsafe fn with_field<R, F: FnOnce(&[u8]) -> R>(key: &str, cb: F) -> R {
    let key = key.as_bytes();
    let written = ::sys::_reason_field(key.as_ptr(), key.len(), REASON_BUFFER.as_mut_ptr());
    cb(&REASON_BUFFER[0..written])
}

pub fn field(key: &str) -> Vec<u8> {
    unsafe { with_field(key, |v| v.to_vec()) }
}

pub fn field_i64(key: &str) -> i64 {
    unsafe { with_field(key, |v| {
        if v.len() != 8 {
            panic!("invalid length")
        }
        *(v.as_ptr() as *const i64)
    }) }
}

pub fn field_string(key: &str) -> String {
    String::from_utf8(field(key)).unwrap()
}