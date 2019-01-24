use std::io::Write;

pub trait Writeable: Sized {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        unsafe {
            let x = ::std::slice::from_raw_parts(::std::mem::transmute(self as *const _ as *const u8), ::std::mem::size_of::<Self>());
            buffer.write_all(x).unwrap();
        }
    }
}

macro_rules! writeable {
    ( $($x:ident), *) => {
        $(
            impl Writeable for $x {}
        )*
    }
}

writeable![usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64, char, bool];

impl Writeable for String {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        for x in self.chars() {
            x.write_to(buffer);
        }

        '\0'.write_to(buffer);
    }
}


impl<T: Writeable> Writeable for Vec<T> {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.len().write_to(buffer);

        for x in self {
            x.write_to(buffer);
        }
    }
}

// Outgoing returned results from a smart contract function call.
pub struct Payload {
    result: Vec<u8>,
}

impl Payload {
    pub fn new() -> Payload {
        Payload { result: vec![] }
    }

    pub fn write<T: Writeable>(&mut self, x: &T) {
        x.write_to(&mut self.result)
    }

    pub fn serialize(self) -> Vec<u8> {
        self.result
    }
}

// Incoming parameters for a smart contract function call.
pub struct Parameters {
    pub transaction_id: Vec<u8>,
    pub sender: Vec<u8>,
    pub amount: u64,

    parameters: Vec<u8>,
    pos: u64,
}

impl Parameters {
    pub fn load() -> Parameters {
        let payload_len = unsafe { crate::sys::_payload_len() };
        let mut payload_bytes = Vec::with_capacity(payload_len);

        unsafe {
            payload_bytes.set_len(payload_len);
            crate::sys::_payload(payload_bytes.as_mut_ptr())
        }

        let mut parameters = Parameters {
            transaction_id: vec![],
            sender: vec![],
            amount: 0,
            parameters: payload_bytes,
            pos: 0,
        };

        parameters.transaction_id = parameters.read();
        parameters.sender = parameters.read();
        parameters.amount = parameters.read();

        parameters
    }

    pub fn read<T: Sized>(&mut self) -> T {
        unsafe {
            let ptr = self.parameters.as_ptr().offset(self.pos as isize);

            let size = ::std::mem::size_of::<T>();

            let x = ::std::slice::from_raw_parts(ptr, size);
            self.pos += size as u64;

            let mut ret: T = ::std::mem::uninitialized();
            ::std::ptr::copy(x.as_ptr(), &mut ret as *mut _ as *mut u8, ::std::mem::size_of::<T>());

            ret
        }
    }
}