#[macro_use]
extern crate serde_derive;

extern crate time;

extern crate serde;
extern crate serde_bench;
extern crate serde_json;

#[macro_use] pub mod enums;

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

pub use std::io::{self, Read, Write};

use std::env;
use std::fs::File;

fn num_trials() -> usize {
    extern crate getopts;
    let mut opts = getopts::Options::new();
    opts.optopt("n", "", "number of trials", "N");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();
    matches.opt_str("n").map(|s| s.parse().unwrap()).unwrap_or(4096)
}

macro_rules! bench_file {
    {
        path: $path:expr,
        structure: $structure:ty,
    } => {
        print!("{:22}", $path);
        io::stdout().flush().unwrap();

        let contents = {
            let mut vec = Vec::new();
            File::open($path).unwrap().read_to_end(&mut vec).unwrap();
            vec
        };

        let data: $structure = serde_json::from_slice(&contents).unwrap();
        let mut binary = Vec::new();
        serde_bench::serialize(&mut binary, &data).unwrap();

        #[cfg(feature = "parse-struct")]
        {
            let dur = timer::bench(num_trials(), || {
                let parsed: $structure = serde_bench::deserialize(&binary).unwrap();
                parsed
            });
            print!("{:6}.{:02}ms", millis(dur), hundredths(dur));
            io::stdout().flush().unwrap();
        }
        #[cfg(not(feature = "parse-struct"))]
        print!("           ");

        #[cfg(feature = "stringify-struct")]
        {
            let len = binary.len();
            let dur = timer::bench_with_buf(num_trials(), len, |out| {
                serde_bench::serialize(out, &data).unwrap()
            });
            print!("{:6}.{:02}ms", millis(dur), hundredths(dur));
            io::stdout().flush().unwrap();
        }

        println!("");
    }
}

fn main() {
    println!("========================== decode | encode ===");

    #[cfg(feature = "file-canada")]
    bench_file! {
        path: "data/canada.json",
        structure: canada::Canada,
    }

    #[cfg(feature = "file-citm-catalog")]
    bench_file! {
        path: "data/citm_catalog.json",
        structure: citm_catalog::CitmCatalog,
    }

    #[cfg(feature = "file-twitter")]
    bench_file! {
        path: "data/twitter.json",
        structure: twitter::Twitter,
    }
}

fn millis(dur: time::Duration) -> i64 {
    dur.num_milliseconds()
}

fn hundredths(dur: time::Duration) -> u8 {
    (dur.num_microseconds().unwrap() / 10 % 100) as u8
}
