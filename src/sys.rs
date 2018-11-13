extern "C" {
    pub fn _reason_len() -> usize;
    pub fn _reason(out: *mut u8);
    pub fn _get_len(key: *const u8, key_len: usize) -> usize;
    pub fn _get(key: *const u8, key_len: usize, value_out: *mut u8);
    pub fn _set(key: *const u8, key_len: usize, value: *const u8, value_len: usize);
    pub fn _send_transaction(
        tag: *const u8,
        tag_len: usize,
        payload: *const u8,
        payload_len: usize,
    );
}
