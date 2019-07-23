//! A sample smart contract that defines a non-fungible token (NFT).
//! Unlike ERC-20 tokens, these tokens are not fungible.
//!
//! An example non-fungible token would be CryptoKitties.

use std::collections::HashMap;

use smart_contract::payload::Parameters;
use smart_contract_macros::smart_contract;

pub struct Ownable {
    ownerships: HashMap<[u8; 32], [u8; 32]>, //LHS:item_id RHS:owner address
}

#[smart_contract]
impl Ownable {
    fn init(_params: &mut Parameters) -> Self {
        //ownerships.insert(params.read::<[u8; 32]>(), params.sender);

        Self {
            ownerships: HashMap::new(),
        }
    }

    fn create_ownable(&mut self, params: &mut Parameters) -> Result<(), String> {
        let proposed_id = params.read::<[u8; 32]>();

        if self.ownerships.contains_key(&proposed_id) {
            return Err("Item is already owned by someone.".into());
        }

        self.ownerships.insert(proposed_id, params.sender);

        Ok(())
    }

    fn transfer_ownership(&mut self, params: &mut Parameters) -> Result<(), String> {
        let recipient = params.read::<[u8; 32]>();
        let item_id = params.read::<[u8; 32]>();

        if !self.ownerships.contains_key(&item_id) {
            return Err("Item is not owned by anyone.".into());
        }

        if self.ownerships.get(&item_id).unwrap() != &recipient {
            return Err("Item is not owned by function caller.".into());
        }

        self.ownerships.insert(item_id, recipient);

        Ok(())
    }
}
