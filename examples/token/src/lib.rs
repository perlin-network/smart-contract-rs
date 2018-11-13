#[macro_use]
extern crate smart_contract;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use smart_contract::utils::token::Token;

pub struct ExampleToken {}

impl Token for ExampleToken {
    fn name() -> &'static str {
        "ExampleToken"
    }
    fn symbol() -> &'static str {
        "WVET"
    }
    fn total_supply() -> u64 {
        0
    }
    fn balance_of(account_id: &str) -> u64 {
        0
    }
    fn transfer(from_id: &str, to_id: &str, amount: u64) {
        panic!()
    }
    fn buy(account_id: &str, n_perls: u64) {
        panic!()
    }
    fn sell(account_id: &str, amount: u64) -> u64 {
        panic!()
    }
}

token_entry!(ExampleToken);
