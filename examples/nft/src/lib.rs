//! A sample smart contract that defines a non-fungible token (NFT)
//! Unlike ERC-20 tokens, it is not fungible
//! CryptoKitties is one of them
use smart_contract::payload::{Parameters, Payload};
use smart_contract_macro::smart_contract;
use std::collections::HashMap;

pub struct Ownable {
    ownerships: HashMap<[u8;32], [u8;32]>,//LHS:item_id RHS:owner address
}

#[smart_contract]
impl Ownable {

    fn init(_params: &mut Parameters) -> Self {
        //ownerships.insert(params.read::<[u8; 32]>(), params.sender);
        
        Self { ownerships : HashMap::new() }
    }
    fn create_ownable(&mut self, params: &mut Parameters) -> Option<Payload> {
        let proposed_id = params.read::<[u8; 32]>();
        if self.ownerships.contains_key(&proposed_id) {
            () 
        }

        self.ownerships.insert(proposed_id,params.sender);
        Some({
            let mut pl = Payload::new();
            pl.write(&true);
            pl
        })
    }

    fn transfer_ownership(&mut self, params: &mut Parameters) -> Option<Payload> {
        // Reading one type only!?
        let recipient = params.read::<[u8; 32]>();
        let item_id = params.read::<[u8; 32]>();
        
        if !self.ownerships.contains_key(&item_id) {
            ()
        }
        if self.ownerships.get(&item_id).unwrap() != &recipient {
            ()
        }
        self.ownerships.insert(item_id,recipient);
        Some({
            let mut pl = Payload::new();
            pl.write(&true);
            pl
        })
    }
}
