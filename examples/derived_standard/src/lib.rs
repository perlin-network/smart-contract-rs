//! A sample token smart contract that extends the base smart
//! contract in `examples/standard` to reward tokens to users
//! who call the `lucky_draw()` function with a transaction ID
//! that has its last bit to be 0.

use std::error::Error;

use smart_contract::payload::Parameters;
use smart_contract_macro::smart_contract;
use standard::Contract;

pub struct Contract2(Contract, u64);


#[smart_contract]
impl Contract2 {
    fn init(params: &mut Parameters) -> Self {
        Contract2(Contract::init(params), 0)
    }

    fn lucky_draw(&mut self, params: &mut Parameters) -> Result<(), Box<dyn Error>> {
        let amount: u64 = params.read();

        self.1 += 1;

        if params.transaction_id[31] == 0 {
            self.0.balances.insert(params.sender, self.0.balances.get(&params.sender).unwrap() + amount);
        }

        Ok(())
    }
}
