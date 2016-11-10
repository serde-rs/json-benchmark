#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate dtoa;
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

#[cfg(feature = "file-canada")]
pub mod canada;
#[cfg(feature = "file-citm-catalog")]
pub mod citm_catalog;
#[cfg(feature = "file-twitter")]
pub mod twitter;

pub use std::io::{self, Read, Write};

use std::env;
use std::fs::File;

#[derive(Copy, Clone)]
pub enum TestFile {
    #[cfg(feature = "file-canada")]
    Canada,
    #[cfg(feature = "file-citm-catalog")]
    CitmCatalog,
    #[cfg(feature = "file-twitter")]
    Twitter,
}

impl TestFile {
    #[allow(unused_mut)]
    pub fn files() -> Vec<TestFile> {
        let mut variants = Vec::new();
        #[cfg(feature = "file-canada")]
        variants.push(TestFile::Canada);
        #[cfg(feature = "file-citm-catalog")]
        variants.push(TestFile::CitmCatalog);
        #[cfg(feature = "file-twitter")]
        variants.push(TestFile::Twitter);
        variants
    }

    pub fn path(self) -> &'static str {
        match self {
            #[cfg(feature = "file-canada")]
            TestFile::Canada => "data/canada.json",
            #[cfg(feature = "file-citm-catalog")]
            TestFile::CitmCatalog => "data/citm_catalog.json",
            #[cfg(feature = "file-twitter")]
            TestFile::Twitter => "data/twitter.json",
        }
    }
}

pub enum Contents {
    #[cfg(feature = "file-canada")]
    Canada(canada::Canada),
    #[cfg(feature = "file-citm-catalog")]
    CitmCatalog(citm_catalog::CitmCatalog),
    #[cfg(feature = "file-twitter")]
    Twitter(twitter::Twitter),
}

fn num_trials() -> usize {
    extern crate getopts;
    let mut opts = getopts::Options::new();
    opts.optopt("n", "", "number of trials", "N");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();
    matches.opt_str("n").map(|s| s.parse().unwrap()).unwrap_or(10)
}

fn main() {
    println!("========================== decode | encode ===");
    for file in TestFile::files() {
        print!("{:22}", file.path());
        io::stdout().flush().unwrap();

        let mut contents = Vec::new();
        File::open(file.path()).unwrap().read_to_end(&mut contents).unwrap();
        let structs = match file {
            #[cfg(feature = "file-canada")]
            TestFile::Canada => {
                let j = serde_json::from_slice(&contents).unwrap();
                contents.clear();
                serde_bench::serialize(&mut contents, &j).unwrap();
                Contents::Canada(j)
            }
            #[cfg(feature = "file-citm-catalog")]
            TestFile::CitmCatalog => {
                let j = serde_json::from_slice(&contents).unwrap();
                contents.clear();
                serde_bench::serialize(&mut contents, &j).unwrap();
                Contents::CitmCatalog(j)
            }
            #[cfg(feature = "file-twitter")]
            TestFile::Twitter => {
                let j = serde_json::from_slice(&contents).unwrap();
                contents.clear();
                serde_bench::serialize(&mut contents, &j).unwrap();
                Contents::Twitter(j)
            }
        };

        let dur = bench_deserialize(contents.as_slice(), file);
        print!("{:6}.{:02}ms", millis(dur), hundredths(dur));
        io::stdout().flush().unwrap();

        let dur = bench_serialize(structs, contents.len());
        print!("{:4}.{:02}ms", millis(dur), hundredths(dur));
        io::stdout().flush().unwrap();

        println!("");
    }
}

fn bench_deserialize(contents: &[u8], which: TestFile) -> time::Duration {
    timer::bench(num_trials(), || {
        match which {
            #[cfg(feature = "file-canada")]
            TestFile::Canada => {
                Contents::Canada(serde_bench::deserialize(contents).unwrap())
            }
            #[cfg(feature = "file-citm-catalog")]
            TestFile::CitmCatalog => {
                Contents::CitmCatalog(serde_bench::deserialize(contents).unwrap())
            }
            #[cfg(feature = "file-twitter")]
            TestFile::Twitter => {
                Contents::Twitter(serde_bench::deserialize(contents).unwrap())
            }
        }
    })
}

fn bench_serialize(contents: Contents, len: usize) -> time::Duration {
    timer::bench_with_buf(num_trials(), len, |out| {
        match contents {
            #[cfg(feature = "file-canada")]
            Contents::Canada(ref v) => serde_bench::serialize(out, v),
            #[cfg(feature = "file-citm-catalog")]
            Contents::CitmCatalog(ref v) => serde_bench::serialize(out, v),
            #[cfg(feature = "file-twitter")]
            Contents::Twitter(ref v) => serde_bench::serialize(out, v),
        }.unwrap();
    })
}

fn millis(dur: time::Duration) -> i64 {
    dur.num_milliseconds()
}

fn hundredths(dur: time::Duration) -> u8 {
    (dur.num_microseconds().unwrap() / 10 % 100) as u8
}
