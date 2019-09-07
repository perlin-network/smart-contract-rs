//! A simple smart contract to create a cryptocurrency token in which
//! people may transact with/keep.
//!
//! We have purposely set up the smart contract to look akin to
//! the likes of an ERC20 Token on Ethereum.
//!
//! Feel free to change up this contract and deploy your own
//! cryptocurrency token on Wavelet!
use std::collections::HashMap;

use smart_contract::debug;
use smart_contract::payload::Parameters;
use smart_contract_macros::smart_contract;

pub struct Token {
    balances: HashMap<[u8; 32], u64>,
}

#[smart_contract]
impl Token {
    fn init(params: &mut Parameters) -> Self {
        let mut balances = HashMap::new();

        balances.insert(params.sender, 100_000);

        debug!(&balances);

        Self { balances }
    }

    fn balance(&mut self, params: &mut Parameters) -> Result<(), String> {
        let wallet_address: [u8; 32] = params.read();

        let wallet_balance = match self.balances.get(&wallet_address) {
            Some(balance) => *balance,
            None => 0,
        };

        debug!(wallet_address, wallet_balance);

        Ok(())
    }

    fn transfer(&mut self, params: &mut Parameters) -> Result<(), String> {
        let sender = params.sender;

        let recipient: [u8; 32] = params.read();
        let amount: u64 = params.read();

        let sender_balance = match self.balances.get(&sender) {
            Some(balance) => *balance,
            None => 0,
        };

        // Throw an error if the sender does not have enough balance.
        if sender_balance < amount {
            return Err("Sender does not have enough balance.".into());
        }

        // Modify sender balance.
        self.balances.insert(sender, sender_balance - amount);

        let recipient_balance = match self.balances.get(&recipient) {
            Some(balance) => *balance,
            None => 0,
        };

        // Modify recipient balance.
        self.balances.insert(recipient, recipient_balance + amount);

        Ok(())
    }
}