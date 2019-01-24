extern "C" {
    pub fn _sender_id_len() -> usize;
    pub fn _sender_id(out: *mut u8);

    pub fn _payload_len() -> usize;
    pub fn _payload(out: *mut u8);

    pub fn _provide_result(result: *const u8, len: usize);

    pub fn _send_transaction(
        tag: *const u8,
        tag_len: usize,
        payload: *const u8,
        payload_len: usize,
    );
}
