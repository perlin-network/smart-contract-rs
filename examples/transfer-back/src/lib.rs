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

#[no_mangle]
pub extern "C" fn contract_main() {
    let reason: Reason<TransferActivation> = match Reason::load() {
        Some(v) => v,
        None => return
    };
    Transaction::new_json(
        "transfer",
        TransferTx {
            amount:
            reason.details.amount,
            recipient: reason.details.sender
        }
    ).send();
}
