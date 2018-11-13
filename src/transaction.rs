#[derive(Clone, Debug)]
pub struct Transaction {
    pub tag: String,
    pub payload: Vec<u8>,
}

#[derive(Serialize)]
pub struct TransferTx<'a> {
    pub amount: u64,
    pub recipient: &'a str,
}

impl Transaction {
    pub fn new<K: Into<String>>(tag: K, payload: Vec<u8>) -> Transaction {
        Transaction {
            tag: tag.into(),
            payload: payload,
        }
    }

    pub fn new_json<K: Into<String>, T: ::serde::Serialize>(tag: K, payload: T) -> Transaction {
        Transaction {
            tag: tag.into(),
            payload: ::serde_json::to_vec(&payload).unwrap(),
        }
    }

    pub fn send(self) {
        let tag = self.tag.as_bytes();
        let payload = self.payload.as_slice();

        unsafe {
            ::sys::_send_transaction(tag.as_ptr(), tag.len(), payload.as_ptr(), payload.len())
        };
    }
}

pub fn transfer(recipient: &str, amount: u64) {
    Transaction::new_json(
        "transfer",
        TransferTx {
            amount: amount,
            recipient: recipient,
        },
    )
    .send();
}
