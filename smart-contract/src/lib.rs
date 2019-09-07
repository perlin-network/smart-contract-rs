#![no_std]

extern crate alloc;

use alloc::fmt::{Debug, Formatter, Result};

pub mod crypto;
pub mod payload;
pub mod sys;
pub mod transaction;

pub fn log(msg: &str) {
    unsafe {
        let msg = msg.as_bytes();
        sys::_log(msg.as_ptr(), msg.len());
    }
}

#[allow(dead_code)]
#[doc(hidden)]
pub struct WrapDebug<T>(pub T);

impl<T: Debug> Debug for WrapDebug<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.fmt(f)
    }
}

#[macro_export]
macro_rules! debug {
    // Handle `debug!()` <-- literal
    () => {
        debug!( () );
    };
    // Handle trailing comma:
    ($($val: expr),+,) => {
        debug!( $($val),+ )
    };
    ($($lab: expr => $val: expr),+,) => {
        debug!( $($lab => $val),+ )
    };
    // Without label, use source of $val:
    ($valf: expr $(, $val: expr)*) => {{
        // in order: for panics, clarification on: debug!(expr);, debug!(expr)
        #[allow(unreachable_code, unused_must_use, unused_parens)]
        let _r = {
            {

            use ::std::io::Write;
            let mut buf = ::std::io::BufWriter::new(vec![]);

            write!(&mut buf, "[{}:{}] ", file!(), line!()).unwrap();

            // Foreach label and expression:
            //     1. Evaluate each expression,
            //     2. Print out $lab = value of expression
            let _ret = (
                {
                    // Evaluate, tmp is value:
                    let _tmp = $crate::WrapDebug($valf);
                    // Won't get further if $val panics.

                    // Print out $lab = _tmo:
                    write!(&mut buf, "{} = {:?}", stringify!($valf), _tmp).unwrap();

                    // Yield tmp:
                    _tmp.0
                }
                $(, {
                    // Comma separator:
                    write!(&mut buf, ", ").unwrap();

                    // Print out $lab = :
                    write!(&mut buf, "{} = ", stringify!($val)).unwrap();

                    // Evaluate, tmp is value:
                    let _tmp = $crate::WrapDebug($val);
                    // Won't get further if $val panics.

                    // Print out tmp:
                    write!(&mut buf, "{:?}" , _tmp).unwrap();

                    // Yield tmp:
                    _tmp.0
                } )*
            );

            $crate::log(::std::str::from_utf8(&buf.into_inner().unwrap()).unwrap());

            // Return the expression:
            _ret
        }
        };
        _r
    }};
    // With label:
    ($labf: expr => $valf: expr $(, $lab: expr => $val: expr)*) => {{
        // in order: for panics, clarification on: debug!(expr);, debug!(expr)
        #[allow(unreachable_code, unused_must_use, unused_parens)]
        let _r = {
            {
            use ::std::io::Write;
            let mut buf = ::std::io::BufWriter::new(vec![]);

            write!(&mut buf, "[{}:{}] ", file!(), line!());

            // Foreach label and expression:
            //     1. Evaluate each expression,
            //     2. Print out $lab = value of expression
            let _ret = (
                {
                    // Enforce is_literal_string($lab):
                    let _ = concat!($labf, "");
                    let _ : &'static str = $labf;

                    // Print out $lab = :
                    write!(&mut buf, "{} = ", stringify!($labf)).unwrap();

                    // Evaluate, tmp is value:
                    let _tmp = $crate::WrapDebug($valf);
                    // Won't get further if $val panics.

                    // Print out tmp:
                    write!(&mut buf, "{:?}" , _tmp).unwrap();

                    // Yield tmp:
                    _tmp.0
                }
                $(, {
                    // Comma separator:
                    write!(&mut buf, ", ").unwrap();

                    // Enforce is_literal_string($lab):
                    let _ = concat!($lab, "");
                    let _ : &'static str = $lab;

                    // Print out $lab = :
                    write!(&mut buf, "{} = ", stringify!($lab)).unwrap();

                    // Evaluate, tmp is value:
                    let _tmp = $crate::WrapDebug($val);
                    // Won't get further if $val panics.

                    // Print out tmp:
                    write!(&mut buf, "{:?}" , _tmp).unwrap();

                    // Yield tmp:
                    _tmp.0
                } )*
            );

            $crate::log(::std::str::from_utf8(&buf.into_inner().unwrap()).unwrap());

            // Return the expression:
            _ret
        }
        };
        _r
    }};
}
