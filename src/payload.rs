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

pub trait Readable<T: Sized> {
    fn read_from(buffer: &Vec<u8>, pos: &mut u64) -> T {
        unsafe {
            let ptr = buffer.as_ptr().offset(*pos as isize);

            let size = ::std::mem::size_of::<T>();

            let x = ::std::slice::from_raw_parts(ptr, size);
            *pos += size as u64;

            let mut ret: T = ::std::mem::uninitialized();
            ::std::ptr::copy(x.as_ptr(), &mut ret as *mut _ as *mut u8, ::std::mem::size_of::<T>());

            ret
        }
    }
}

macro_rules! readable {
    ( $($x:ident), *) => {
        $(
            impl Readable<$x> for $x {}
        )*
    }
}

readable![usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64, char, bool];

impl Readable<String> for String {
    fn read_from(buffer: &Vec<u8>, pos: &mut u64) -> String {
        let mut buf = vec![];

        loop {
            let chr = char::read_from(buffer, pos);

            if chr == '\0' {
                break;
            }

            buf.push(chr);
        }

        buf.into_iter().collect()
    }
}

impl<U: Readable<U>> Readable<Vec<U>> for Vec<U> {
    fn read_from(buffer: &Vec<u8>, pos: &mut u64) -> Vec<U> {
        let mut buf: Vec<U> = vec![];
        let len = usize::read_from(buffer, pos);

        for _ in 0..len {
            buf.push( U::read_from(buffer, pos));
        }

        buf
    }
}

// Outgoing returned results from a smart contract function call.
pub struct Payload {
    result: Vec<u8>,
}

impl From<Vec<u8>> for Payload {
    fn from(params: Vec<u8>) -> Self {
        Payload { result: params }
    }
}

impl Payload {
    pub fn new() -> Payload {
        Payload { result: vec![] }
    }

    pub fn write<T: Writeable>(&mut self, x: &T) {
        x.write_to(&mut self.result)
    }

    pub fn serialize(&self) -> &Vec<u8> {
        &self.result
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

    pub fn read<T: Readable<T>>(&mut self) -> T {
        T::read_from(&self.parameters, &mut self.pos)
    }
}