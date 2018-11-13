pub trait Token {
    fn name() -> &'static str;
    fn symbol() -> &'static str;
    fn total_supply() -> u64;
    fn balance_of(account_id: &str) -> u64;
    fn transfer(from_id: &str, to_id: &str, amount: u64);
    fn buy(account_id: &str, n_perls: u64);
    fn sell(account_id: &str, amount: u64) -> u64 /* perls */;
}

#[macro_export]
macro_rules! token_entry {
    ($entry:ident) => {
        use $crate::activation::{CustomActivation, TransferActivation};
        use $crate::serde_json;
        use $crate::transaction;
        use $crate::Reason;

        #[derive(Deserialize)]
        pub enum TokenActivationPayload {
            TransferTo { recipient: String, amount: u64 },
            Sell { amount: u64 },
        }

        #[no_mangle]
        pub extern "C" fn contract_main() {
            fn assert_entry_type<T: Token>() {}
            //assert_entry_type::<$entry>();

            let reason: Option<Reason<Box<serde_json::value::RawValue>>> = Reason::load();
            match reason {
                Some(reason) => match reason.kind.as_str() {
                    "transfer" => {
                        let activation: TransferActivation =
                            match serde_json::from_str(reason.details.get()) {
                                Ok(v) => v,
                                Err(_) => return,
                            };
                        let sender = reason.sender;
                        $entry::buy(sender.as_str(), activation.amount);
                    }
                    "custom" => {
                        let activation: CustomActivation<TokenActivationPayload> =
                            match serde_json::from_str(reason.details.get()) {
                                Ok(v) => v,
                                Err(_) => return,
                            };
                        let sender = reason.sender;
                        let payload = activation.body;
                        match payload {
                            TokenActivationPayload::TransferTo { recipient, amount } => {
                                $entry::transfer(sender.as_str(), recipient.as_str(), amount);
                            }
                            TokenActivationPayload::Sell { amount } => {
                                let n_perls = $entry::sell(sender.as_str(), amount);
                                transaction::transfer(sender.as_str(), n_perls);
                            }
                        }
                    }
                    _ => {}
                },
                None => {}
            }
        }
    };
}
