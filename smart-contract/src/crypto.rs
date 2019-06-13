#[derive(Copy, Clone, Debug)]
pub enum SignatureAlgorithm {
    Ed25519,
}

#[derive(Copy, Clone, Debug)]
pub enum HashAlgorithm {
    Blake2b256,
    Blake2b512,
    Sha256,
    Sha512,
}

pub const BLAKE2B256_OUTPUT_SIZE: usize = 32;
pub const BLAKE2B512_OUTPUT_SIZE: usize = 64;
pub const SHA256_OUTPUT_SIZE: usize = 32;
pub const SHA512_OUTPUT_SIZE: usize = 64;

pub fn verify(alg: SignatureAlgorithm, pubkey: &[u8], data: &[u8], sig: &[u8]) -> Result<(), ()> {
    match alg {
        SignatureAlgorithm::Ed25519 => unsafe {
            match crate::sys::_verify_ed25519(
                pubkey.as_ptr(),
                pubkey.len(),
                data.as_ptr(),
                data.len(),
                sig.as_ptr(),
                sig.len(),
            ) {
                0 => Ok(()),
                _ => Err(()),
            }
        },
    }
}

pub fn hash(alg: HashAlgorithm, data: &[u8], out: &mut [u8]) -> Result<(), ()> {
    let f = match alg {
        HashAlgorithm::Blake2b256 => crate::sys::_hash_blake2b_256,
        HashAlgorithm::Blake2b512 => crate::sys::_hash_blake2b_512,
        HashAlgorithm::Sha256 => crate::sys::_hash_sha256,
        HashAlgorithm::Sha512 => crate::sys::_hash_sha512,
    };
    unsafe {
        match f(data.as_ptr(), data.len(), out.as_mut_ptr(), out.len()) {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}
