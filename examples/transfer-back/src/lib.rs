#[macro_use]
extern crate smart_contract;
extern crate serde;

use smart_contract::activation::TransferActivation;
use smart_contract::transaction::transfer;
use smart_contract::Reason;

fn handle_activation() {
    let reason: Reason<TransferActivation> = match Reason::load() {
        Some(v) => v,
        None => return,
    };

    transfer(&reason.details.sender, (reason.details.amount + 1) / 2);
}

contract_entry!(handle_activation);
