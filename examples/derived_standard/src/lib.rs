//! A sample token smart contract that extends the base smart
//! contract in `examples/standard` to reward tokens to users
//! who call the `lucky_draw()` function with a transaction ID
//! that has its last bit to be 0.

use smart_contract::payload::Parameters;
use smart_contract_macro::smart_contract;
use standard::Contract;
use std::error::Error;
use std::fmt;

pub struct Contract2(Contract, u64);

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
impl Contract2 {
    fn init(params: &mut Parameters) -> Self {
        Contract2(Contract::init(params), 0)
    }

    fn lucky_draw(&mut self, params: &mut Parameters) -> Result<(), Box<dyn Error>> {
        let amount: u64 = params.read();
        if amount > 10 {
            return Err(Box::new(LuckyDrawError::AmountTooHigh));
        }

        if self.1 == ::std::u64::MAX {
            return Err(Box::new(LuckyDrawError::CustomError(
                "Counter is full".to_string(),
            )));
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
