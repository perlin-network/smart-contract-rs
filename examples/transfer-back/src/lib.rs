#[macro_use]
extern crate smart_contract;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use smart_contract::activation::TransferActivation;
use smart_contract::transaction::transfer;
use smart_contract::{Reason, Transaction};

#[derive(Serialize)]
pub struct TransferTx {
    pub amount: u64,
    pub recipient: String,
}

fn handle_activation() {
    let reason: Reason<TransferActivation> = match Reason::load() {
        Some(v) => v,
        None => return,
    };

    transfer(&reason.details.sender, (reason.details.amount + 1) / 2);
}

contract_entry!(handle_activation);
