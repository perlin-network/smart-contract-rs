extern crate serde;
#[macro_use]
extern crate serde_derive;
pub extern crate serde_json;

pub mod activation;
pub mod persistent;
pub mod reason;
pub mod sys;
pub mod transaction;
pub mod utils;

pub use reason::Reason;
pub use transaction::Transaction;

#[macro_export]
macro_rules! contract_entry {
    ($entry:ident) => {
        #[no_mangle]
        pub extern "C" fn contract_main() {
            fn assert_entry_type<F: Fn()>(_: F) {}
            assert_entry_type($entry);

            $entry();
        }
    };
}
