use std::io::Write;

pub trait Writeable: Sized {
    fn write_to(&self, buffer: &mut Vec<u8>);
}

macro_rules! writeable {
    ( $($x:ident), *) => {
        $(
            impl Writeable for $x {
                fn write_to(&self, buffer: &mut Vec<u8>) {
                    unsafe {
                        let x = ::std::slice::from_raw_parts(self as *const _ as *const u8, ::std::mem::size_of::<Self>());
                        buffer.write_all(x).unwrap();
                    }
                }
            }
        )*
    }
}

macro_rules! writeable_array {
    ( $n:expr) => {
        impl<U: Writeable> Writeable for [U; $n] {
            fn write_to(&self, buffer: &mut Vec<u8>) {
                for i in self {
                    i.write_to(buffer);
                }
            }
        }
    };
}

writeable![usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64];
writeable_array![32];

impl Writeable for bool {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        if *self {
            1u8.write_to(buffer);
        } else {
            0u8.write_to(buffer);
        }
    }
}

impl Writeable for String {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        for x in self.chars() {
            (x as u8).write_to(buffer);
        }

        0u8.write_to(buffer);
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

pub trait Readable {
    fn read_from(buffer: &[u8], pos: &mut u64) -> Self;
}

macro_rules! readable {
    ( $($x:ident), *) => {
        $(
            impl Readable for $x {
                fn read_from(buffer: &[u8], pos: &mut u64) -> Self {
                    unsafe {
                        let ptr = buffer.as_ptr().offset(*pos as isize);

                        let size = ::std::mem::size_of::<$x>();

                        let x = ::std::slice::from_raw_parts(ptr, size);
                        *pos += size as u64;

                        let mut ret: $x = ::std::mem::uninitialized();
                        ::std::ptr::copy(x.as_ptr(), &mut ret as *mut _ as *mut u8, ::std::mem::size_of::<$x>());

                        ret
                    }
                }
            }
        )*
    }
}

macro_rules! readable_array {
    ( $n:expr) => {
        impl<U: Readable + Copy + Default> Readable for [U; $n] {
            fn read_from(buffer: &[u8], pos: &mut u64) -> [U; $n] {
                let mut buf: [U; $n] = [U::default(); $n];

                for i in 0..$n {
                    buf[i] = U::read_from(buffer, pos);
                }

                buf
            }
        }
    };
}

readable![usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64];
readable_array![32];

impl Readable for bool {
    fn read_from(buffer: &[u8], pos: &mut u64) -> bool {
        u8::read_from(buffer, pos) == 1
    }
}

impl Readable for String {
    fn read_from(buffer: &[u8], pos: &mut u64) -> String {
        let mut buf = vec![];

        loop {
            let chr = u8::read_from(buffer, pos);

            if chr == 0 {
                break;
            }

            buf.push(chr);
        }

        String::from_utf8(buf).unwrap()
    }
}

impl<U: Readable> Readable for Vec<U> {
    fn read_from(buffer: &[u8], pos: &mut u64) -> Vec<U> {
        let mut buf: Vec<U> = vec![];
        let len = usize::read_from(buffer, pos);

        for _ in 0..len {
            buf.push(U::read_from(buffer, pos));
        }

        buf
    }
}

// Outgoing returned results from a smart contract function call.
#[derive(Default)]
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

    pub fn serialize(&self) -> &[u8] {
        &self.result
    }
}

// Incoming parameters for a smart contract function call.
#[derive(Default)]
pub struct Parameters {
    pub transaction_id: [u8; 32],
    pub sender: [u8; 32],
    pub amount: u64, // can be extended or removed

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
            transaction_id: [0; 32],
            sender: [0; 32],
            amount: 0,
            parameters: payload_bytes,
            pos: 0,
        };

        parameters.transaction_id = parameters.read();
        parameters.sender = parameters.read();
        parameters.amount = parameters.read();

        parameters
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read_from(&self.parameters, &mut self.pos)
    }
}
