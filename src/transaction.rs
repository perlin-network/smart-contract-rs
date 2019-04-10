use crate::payload::{Payload, Readable, Writeable};

#[repr(u8)]
pub enum TransactionTag {
    Nop,
    Transfer,
    Contract,
    Stake,
}

pub trait Transaction: Writeable + Readable<Self> {
    fn send_transaction(self) {
        let mut payload = vec![];
        self.write_to(&mut payload);

        unsafe {
            crate::sys::_send_transaction(self.tag() as u8, payload.as_ptr(), payload.len());
        }
    }

    #[inline]
    fn tag(&self) -> TransactionTag;
}

pub struct Transfer {
    pub destination: [u8; 32],
    pub amount: u64,

    pub func_name: Vec<u8>,
    pub func_params: Vec<u8>,
}

impl Writeable for Transfer {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.destination.write_to(buffer);
        self.amount.write_to(buffer);

        self.func_name.write_to(buffer);
        self.func_params.write_to(buffer);
    }
}

impl Readable<Transfer> for Transfer {
    fn read_from(buffer: &Vec<u8>, pos: &mut u64) -> Transfer {
        Transfer {
            destination: <[u8; 32]>::read_from(buffer, pos),
            amount: u64::read_from(buffer, pos),

            func_name: Vec::<u8>::read_from(buffer, pos),
            func_params: Vec::<u8>::read_from(buffer, pos),
        }
    }
}

impl Transaction for Transfer {
    fn tag(&self) -> TransactionTag {
        TransactionTag::Transfer
    }
}

pub struct Contract {
    pub contract_id: Vec<u8>,
    pub payload: crate::payload::Payload,
}

impl Writeable for Contract {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.contract_id.write_to(buffer);
        self.payload.serialize().write_to(buffer);
    }
}

impl Readable<Contract> for Contract {
    fn read_from(buffer: &Vec<u8>, pos: &mut u64) -> Contract {
        Contract {
            contract_id: Vec::<u8>::read_from(buffer, pos),
            payload: Payload::from(Vec::<u8>::read_from(buffer, pos)),
        }
    }
}