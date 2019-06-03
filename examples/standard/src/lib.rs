//! A dummy base smart contract that may be extended.

use std::collections::HashMap;

use smart_contract::payload::Parameters;
use smart_contract_macro::smart_contract;

#[derive(Default)]
pub struct Contract {
    pub balances: HashMap<[u8; 32], u64>,
}

#[smart_contract]
impl Contract {
    pub fn init(params: &mut Parameters) -> Self {
        let mut balances = HashMap::new();
        balances.insert(params.sender, 100_000);
        Self { balances }
    }
}
