//! A simple smart contract that sends half of the PERLs it receives
//! back to its respective sender.
//!
//! Overall a simple example of registering a function to get called when
//! the smart contract receives PERLs, and on how to create and send PERLs
//! to a provided destination wallet address.
use smart_contract::payload::{Parameters, Payload};
use smart_contract::transaction::{Transaction, Transfer};
use smart_contract_macro::smart_contract;

pub struct Contract;

#[smart_contract]
impl Contract {
    fn init(_params: &mut Parameters) -> Self {
        Self{}
    }

    fn on_money_received(&mut self, params: &mut Parameters) -> Option<Payload> {
        // Create and send transaction.
        Transfer {
            destination: params.sender.clone(),
            amount: (params.amount + 1) / 2,
            func_name: vec![],
            func_params: vec![],
        }.send_transaction();

        None
    }
}