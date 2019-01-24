use crate::payload::Writeable;

#[repr(u8)]
pub enum TransactionTag {
    Nop,
    Transfer,
    Contract,
    Stake,
}

pub trait Transaction: Writeable {
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
    pub destination: Vec<u8>,
    pub amount: u64,

    pub func_name: Vec<u8>,
    pub func_params: Vec<u8>
}

pub struct Contract {
    pub contract_id: Vec<u8>,
    pub payload: crate::payload::Payload,
}

macro_rules! transaction {
    ( $($x:ident), *) => {
        $(
            impl Writeable for $x {}
            impl Transaction for $x {
                #[inline]
                fn tag(&self) -> TransactionTag {
                    TransactionTag::$x
                }
            }
        )*
    }
}

transaction![Transfer, Contract];