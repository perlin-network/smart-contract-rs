extern "C" {
    // All out pointers passed to _reason_* methods should point to a block of memory
    // of at least 65536 bytes available for write.
    pub fn _reason_tag(out: *mut u8) -> usize;
    pub fn _reason_field(key: *const u8, key_len: usize, out: *mut u8) -> usize;

    pub fn _get_len(key: *const u8, key_len: usize) -> usize;
    pub fn _get(key: *const u8, key_len: usize, value_out: *mut u8);
    pub fn _set(key: *const u8, key_len: usize, value: *const u8, value_len: usize);
    pub fn _send_transaction(tag: *const u8, tag_len: usize, payload: *const u8, payload_len: usize);
}
