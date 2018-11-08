#[macro_use]
extern crate smart_contract;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use smart_contract::{Reason, Transaction};

#[derive(Deserialize)]
pub struct TransferActivation {
    pub sender: String,
    pub amount: u64,
}

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
    Transaction::new_json(
        "transfer",
        TransferTx {
            amount: (reason.details.amount + 1) / 2,
            recipient: reason.details.sender,
        },
    )
    .send();
}

contract_entry!(handle_activation);
