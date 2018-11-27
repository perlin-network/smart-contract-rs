#[derive(Deserialize)]
pub struct Reason<T> {
    pub kind: String,
    pub sender: Vec<u8>,
    pub details: T,
}

impl<T: for<'a> ::serde::Deserialize<'a>> Reason<T> {
    pub fn load() -> Option<Reason<T>> {
        let raw_len = unsafe { ::sys::_reason_len() };
        let mut raw = Vec::with_capacity(raw_len);
        unsafe { raw.set_len(raw_len) };
        unsafe { ::sys::_reason(raw.as_mut_ptr()) };
        match ::serde_json::from_slice(&raw) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
