//! A simple smart contract to create a cryptocurrency token in which
//! people may transact with/keep.
//!
//! We have purposely set up the smart contract to look akin to
//! the likes of an ERC20 Token on Ethereum.
//!
//! The tokens name in this case is WVL; feel free to change up this contract
//! and deploy your own cryptocurrency token on Wavelet!
use smart_contract::debug;
use smart_contract::payload::{Parameters, Payload};
use smart_contract_macro::smart_contract;
use std::collections::HashMap;

pub struct Token {
    balances: HashMap<Vec<u8>, u64>,
}

#[smart_contract]
impl Token {
    fn init(params: &mut Parameters) -> (Self, Option<Payload>) {
        let mut balances = HashMap::new();

        balances.insert(params.sender.clone(), 100000);

        debug!(&balances);

        (Self { balances }, None)
    }

    fn balance(&mut self, params: &mut Parameters) -> Option<Payload> {
        let wallet_address: Vec<u8> = params.read();

        let mut result = Payload::new();

        let wallet_balance = match self.balances.get(&wallet_address) {
            Some(balance) => *balance,
            None => 0
        };

        debug!(wallet_address, wallet_balance);

        result.write(&wallet_balance);

        Some(result)
    }

    fn transfer(&mut self, params: &mut Parameters) -> Option<Payload> {
        let recipient: Vec<u8> = params.read();
        let amount: u64 = params.read();

        let mut result = Payload::new();

        let sender_balance = match self.balances.get(&params.sender) {
            Some(balance) => *balance,
            None => 0
        };

        // Throw an error if the sender does not have enough balance.
        if sender_balance < amount {
            result.write(&false);
            return Some(result);
        }

        let recipient_balance = match self.balances.get(&recipient) {
            Some(balance) => *balance,
            None => 0
        };

        // Modify balances.
        self.balances.insert(params.sender.clone(), sender_balance - amount);
        self.balances.insert(recipient, recipient_balance + amount);

        // Return `true` to the sender that the transfer was successful.
        result.write(&true);
        return Some(result);
    }
}