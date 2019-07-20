//! This contract recursively invokes itself.
//! DO NOT invoke this contract with real money.
//!
use smart_contract::payload::Parameters;
use smart_contract::transaction::{Transaction, Transfer, Invocation};
use smart_contract_macros::smart_contract;

pub struct Contract;

#[smart_contract]
impl Contract {
    fn init(_params: &mut Parameters) -> Self {
        Self {}
    }

    fn bomb(&mut self, params: &mut Parameters) -> Result<(), String> {
        let id: [u8; 32] = params.read();

        Transfer {
            destination: id,
            amount: 0,
            invocation: Some(Invocation {
                gas_limit: 1000000,
                gas_deposit: 0,
                func_name: b"bomb".to_vec(),
                func_params: id.to_vec(),
            })
        }
        .send_transaction();

        Ok(())
    }
}
