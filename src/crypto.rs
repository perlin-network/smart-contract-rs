extern "C" {
    pub fn _sign(key: *const u8, key_len: usize, msg: *const u8, msg_len: usize, sign_out: *mut u8);
    pub fn _verify(pubkey: *const u8, pubkey_len: usize, msg: *const u8, msg_len: usize, sign: *const u8, sign_len: usize) -> usize;
    pub fn _hash(data: *const u8, data_len: usize, hash_out: *mut u8);
}