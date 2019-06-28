#[macro_use]
pub mod enums;

pub mod adapter;
pub mod color;
pub mod empty;
pub mod prim_str;
pub mod timer;

#[cfg(feature = "zero-copy")]
pub mod borrow;
#[cfg(feature = "zero-copy")]
pub use self::borrow::*;

#[cfg(not(feature = "zero-copy"))]
pub mod copy;
#[cfg(not(feature = "zero-copy"))]
pub use self::copy::*;

#[cfg(feature = "file-canada")]
pub mod canada;

use std::env;

pub fn num_trials() -> Option<usize> {
    let mut opts = getopts::Options::new();
    opts.optopt("n", "", "number of trials", "N");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();
    matches.opt_str("n").map(|s| s.parse().unwrap())
}

pub fn throughput(dur: time::Duration, bytes: usize) -> u64 {
    let mut megabytes_per_second = bytes as u64 / dur.num_microseconds().unwrap() as u64;

    // Round to two significant digits.
    if megabytes_per_second > 100 {
        if megabytes_per_second % 10 >= 5 {
            megabytes_per_second += 10;
        }
        megabytes_per_second = megabytes_per_second / 10 * 10;
    }

    megabytes_per_second
}
