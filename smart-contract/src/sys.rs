extern "C" {
    pub fn _payload_len() -> usize;
    pub fn _payload(out: *mut u8);

    pub fn _log(content: *const u8, len: usize);
    pub fn _result(result: *const u8, len: usize);

    pub fn _send_transaction(tag: u8, payload: *const u8, payload_len: usize);

    pub fn _verify_ed25519(
        pubkey: *const u8,
        pubkey_len: usize,
        data: *const u8,
        data_len: usize,
        sig: *const u8,
        sig_len: usize,
    ) -> i32;

    pub fn _hash_blake2b_256(data: *const u8, data_len: usize, out: *mut u8, out_len: usize)
        -> i32;

    pub fn _hash_blake2b_512(data: *const u8, data_len: usize, out: *mut u8, out_len: usize)
        -> i32;

    pub fn _hash_sha256(data: *const u8, data_len: usize, out: *mut u8, out_len: usize) -> i32;

    pub fn _hash_sha512(data: *const u8, data_len: usize, out: *mut u8, out_len: usize) -> i32;
}
