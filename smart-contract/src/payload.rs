use std::io::Write;

pub trait Writeable {
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

writeable![usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64];
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

impl Writeable for str {
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

impl<T: Writeable> Writeable for [T] {
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

readable![usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64];
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

// Incoming parameters for a smart contract function call.
#[derive(Default)]
pub struct Parameters {
    pub round_idx: u64,
    pub round_id: [u8; 32],
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

        let mut parameters = Parameters::default();
        parameters.parameters = payload_bytes;

        parameters.round_idx = parameters.read();
        parameters.round_id = parameters.read();
        parameters.transaction_id = parameters.read();
        parameters.sender = parameters.read();
        parameters.amount = parameters.read();

        parameters
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read_from(&self.parameters, &mut self.pos)
    }
}

pub struct ParametersBuilder {
    params: Parameters,
}

#[derive(Clone, Debug)]
pub struct ParametersBuilderConfig {
    pub round_idx: u64,
    pub round_id: [u8; 32],
    pub transaction_id: [u8; 32],
    pub sender: [u8; 32],
    pub amount: u64, // can be extended or removed
}

impl ParametersBuilder {
    pub fn new(config: ParametersBuilderConfig) -> ParametersBuilder {
        ParametersBuilder {
            params: Parameters {
                round_idx: config.round_idx,
                round_id: config.round_id,
                transaction_id: config.transaction_id,
                sender: config.sender,
                amount: config.amount, 
                parameters: vec![],
                pos: 0,
            }
        }
    }

    pub fn write<T: Writeable + ?Sized>(&mut self, x: &T) {
        x.write_to(&mut self.params.parameters);
    }

    pub fn build(self) -> Parameters {
        self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters_builder() {
        const ROUND_IDX: u64 = 100;
        const ROUND_ID: [u8; 32] = [42; 32];
        const TRANSACTION_ID: [u8; 32] = [0; 32];
        const SENDER: [u8; 32] = [1; 32];
        const AMOUNT: u64 = 20;

        let mut builder = ParametersBuilder::new(ParametersBuilderConfig {
            round_idx: ROUND_IDX,
            round_id: ROUND_ID,
            transaction_id: TRANSACTION_ID,
            sender: SENDER,
            amount: AMOUNT,
        });
        builder.write(&100u64);
        builder.write("Hello");

        let mut params = builder.build();

        assert_eq!(params.round_idx, ROUND_IDX);
        assert_eq!(params.round_id, ROUND_ID);
        assert_eq!(params.transaction_id, TRANSACTION_ID);
        assert_eq!(params.sender, SENDER);
        assert_eq!(params.amount, AMOUNT);

        assert_eq!(params.read::<u64>(), 100);
        assert_eq!(params.read::<String>(), "Hello");
    }
}