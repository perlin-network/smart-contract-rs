//! A sample of token that can extends its impl functions
//! 

use smart_contract::payload::{Parameters, Payload};
use smart_contract_macro::smart_contract;
use standard::Contract;

pub struct Contract2(Contract,u64);


#[smart_contract]
impl Contract2 {
    fn init(params: &mut Parameters) -> Self {
        Contract2(Contract::init(params),0)
    }
    
    fn lucky_draw(&mut self, params: &mut Parameters) -> Option<Payload> {
        let amount: u64 = params.read();
        self.1 += 1;
        if params.transaction_id[31] == 0 {
            self.0.balances.insert(params.sender, self.0.balances.get(&params.sender).unwrap() + amount);
        }
        None
    }

}
