//! A simple decentralized chat example where all messages get pruned after they remain
//! within the contract over a period of 50 consensus rounds.

use std::collections::BTreeMap;
use std::error::Error;

use smart_contract::log;
use smart_contract::payload::Parameters;
use smart_contract_macros::smart_contract;

struct Entry {
    sender: [u8; 32],
    message: String,
}

struct Chat {
    logs: BTreeMap<u64, Vec<Entry>>
}

const NUM_ROUNDS_UNTIL_MESSAGE_PRUNED: u64 = 50;

fn prune_old_messages(chat: &mut Chat, current_round_idx: u64) {
    let pruned_round_indices: Vec<u64> = chat.logs.iter().filter_map(|(round_idx, _)| {
        if round_idx + NUM_ROUNDS_UNTIL_MESSAGE_PRUNED < current_round_idx {
            Some(*round_idx)
        } else {
            None
        }
    }).collect();

    for round_idx in pruned_round_indices {
        chat.logs.remove(&round_idx);
    }
}

#[smart_contract]
impl Chat {
    fn init(_params: &mut Parameters) -> Self {
        Self { logs: BTreeMap::new() }
    }

    fn send_message(&mut self, params: &mut Parameters) -> Result<(), Box<dyn Error>> {
        let entry = Entry { sender: params.sender, message: params.read() };

        if let Some(entries) = self.logs.get_mut(&params.round_idx) {
            entries.push(entry);
            return Ok(());
        }

        self.logs.insert(params.round_idx, vec![entry]);

        prune_old_messages(self, params.round_idx);

        Ok(())
    }

    fn get_messages(&mut self, _params: &mut Parameters) -> Result<(), Box<dyn Error>> {
        let mut messages = Vec::new();

        for (_, logs) in self.logs.iter_mut() {
            for entry in logs {
                messages.push(format!("<{:x?}> {}", entry.sender, entry.message));
            }
        }

        log(&messages.join("\n"));

        Ok(())
    }
}