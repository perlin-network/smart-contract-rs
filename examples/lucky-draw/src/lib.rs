//! A sample token smart contract that extends a base smart contract
//! to reward tokens to users who call the `lucky_draw()` function
//! with a transaction ID that has its last bit to be 0.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use smart_contract::payload::Parameters;
use smart_contract_macros::smart_contract;

pub struct Contract {
    pub balances: HashMap<[u8; 32], u64>,
}

impl Contract {
    pub fn init(params: &mut Parameters) -> Self {
        let mut balances = HashMap::new();
        balances.insert(params.sender, 100_000);
        Self { balances }
    }
}

pub struct LuckyDraw(Contract, u64);

#[derive(Debug, Clone)]
enum LuckyDrawError {
    AmountTooHigh,
    CustomError(String),
}

impl Error for LuckyDrawError {}

impl fmt::Display for LuckyDrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LuckyDrawError::AmountTooHigh => write!(f, "No rewards more than 10 is allowed."),
            LuckyDrawError::CustomError(ref cause) => write!(f, "Error: {}", cause),
        }
    }
}

#[smart_contract]
impl LuckyDraw {
    fn init(params: &mut Parameters) -> Self {
        LuckyDraw(Contract::init(params), 0)
    }

    fn lucky_draw(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        if amount > 10 {
            return Err(LuckyDrawError::AmountTooHigh.to_string());
        }

        if self.1 == ::std::u64::MAX {
            return Err(LuckyDrawError::CustomError("Counter is full".to_string()).to_string());
        }
        self.1 += 1;

        if params.transaction_id[31] == 0 {
            self.0.balances.insert(
                params.sender,
                self.0.balances.get(&params.sender).unwrap() + amount,
            );
        }

        Ok(())
    }
}
