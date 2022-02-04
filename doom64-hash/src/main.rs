#![warn(clippy::all)]
use doom64_hash::*;
use std::env;

/*
fn u16_signed(n: u16) -> (u16, bool) {
    let mut signed = n & (i16::MAX as u16);
    let negative = signed != n;
    if negative {
        signed ^= i16::MAX as u16;
    }
    (signed, negative)
}
*/

fn main() {
    let mut args = env::args();
    let unsigned = env::var("HASH_SIGNED").is_err() && env::var("CSV").is_err();
    let csv = env::var("CSV").is_ok();
    args.next(); // Program name

    if csv {
        println!("name,uhash,hash");
    }
    args.for_each(|argstr| {
        let uhash = hash(&argstr);
        let hash = if unsigned {
            format!("{}", uhash)
        } else {
            format!("{}", uhash as i16)
        };
        if csv {
            println!("{},{},{}", argstr, uhash, hash);
        } else {
            println!("{}", hash);
        }
    });
}
