extern "C" {
    pub fn _payload_len() -> usize;
    pub fn _payload(out: *mut u8);

    pub fn _log(content: *const u8, len: usize);
    pub fn _result(result: *const u8, len: usize);

    pub fn _send_transaction(
        tag: u8,
        payload: *const u8,
        payload_len: usize,
    );
}