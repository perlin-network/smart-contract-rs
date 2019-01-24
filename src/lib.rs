pub mod payload;
pub mod sys;
pub mod transaction;

pub fn log(msg: &str) {
    unsafe {
        let msg = msg.as_bytes();
        sys::_log(msg.as_ptr(), msg.len());
    }
}
