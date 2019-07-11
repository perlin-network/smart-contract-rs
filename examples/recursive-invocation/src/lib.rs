//! This contract ecursively invokes itself.
//! DO NOT invoke this contract with real money.
use std::error::Error;

use smart_contract::payload::Parameters;
use smart_contract::transaction::{Transaction, Transfer};
use smart_contract_macros::smart_contract;

pub struct Contract;

#[smart_contract]
impl Contract {
    fn init(_params: &mut Parameters) -> Self {
        Self {}
    }

    fn bomb(&mut self, params: &mut Parameters) -> Result<(), Box<dyn Error>> {
        // Create and send transaction.
        let id: [u8; 32] = params.read();
        Transfer {
            destination: id,
            amount: 0,
            func_name: b"bomb".to_vec(),
            func_params: id.to_vec(),
        }
        .send_transaction();

        Ok(())
    }
}
