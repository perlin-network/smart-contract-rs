use crate::payload::Writeable;

pub trait Transaction: Writeable {
    fn send_transaction(&self, tag: &str) {
        let mut payload = vec![];
        self.write_to(&mut payload);

        unsafe {
            crate::sys::_send_transaction(tag.as_ptr(), tag.len(), payload.as_ptr(), payload.len());
        }
    }
}

pub struct Transfer {
    pub destination: Vec<u8>,
    pub amount: u64,
}

pub struct Contract {
    pub contract_id: Vec<u8>,
    pub payload: crate::payload::Payload,
}

macro_rules! transaction {
    ( $($x:ident), *) => {
        $(
            impl Writeable for $x {}
            impl Transaction for $x {}
        )*
    }
}

transaction![Transfer, Contract];