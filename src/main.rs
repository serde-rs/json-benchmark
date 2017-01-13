#![feature(stmt_expr_attributes)]

#[cfg(feature = "lib-serde")]
#[macro_use]
extern crate serde_derive;

extern crate dtoa;
extern crate time;

#[cfg(feature = "lib-rustc-serialize")]
extern crate rustc_serialize;
#[cfg(feature = "lib-serde")]
extern crate serde;

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

trait Library {
    type DOM;

    fn name() -> &'static str;

    fn parse_dom<R>(read: R) -> Result<Self::DOM, ()>
        where R: io::Read;
    fn stringify_dom<W>(write: &mut W, dom: &Self::DOM)
        where W: io::Write;

    fn can_codegen() -> bool { false }
    fn parse_struct<R>(R, TestFile) -> Contents
        where R: io::Read { unimplemented!() }
    fn stringify_struct<W>(&mut W, &Contents)
        where W: io::Write { unimplemented!() }
}

fn main() {
    print!("{:>35}{:>22}", "DOM", "STRUCT");
    #[cfg(feature = "lib-serde")]
    bench::<serde_json::Library>();
    #[cfg(feature = "lib-json-rust")]
    bench::<json_rust::Library>();
    #[cfg(feature = "lib-rustc-serialize")]
    bench::<rustc::Library>();
}

fn num_trials() -> usize {
    extern crate getopts;
    let mut opts = getopts::Options::new();
    opts.optopt("n", "", "number of trials", "N");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();
    matches.opt_str("n").map(|s| s.parse().unwrap()).unwrap_or(10)
}

fn bench<L: Library>() {
    let name = format!(" {} ", L::name());
    println!("\n{:=^26} parse|stringify === parse|stringify ===", name);
    for file in TestFile::files() {
        print!("{:22}", file.path());
        io::stdout().flush().unwrap();

        let mut contents = Vec::new();
        File::open(file.path()).unwrap().read_to_end(&mut contents).unwrap();
        let dom = L::parse_dom(contents.as_slice()).unwrap();

        if cfg!(feature = "parse-dom") {
            let dur = bench_parse_dom::<L>(contents.as_slice());
            print!("{:6}.{}ms", millis(dur), tenths(dur));
            io::stdout().flush().unwrap();
        } else {
            print!("          ");
        }

        if cfg!(feature = "stringify-dom") {
            let dur = bench_stringify_dom::<L>(&dom, contents.len());
            print!("{:6}.{}ms", millis(dur), tenths(dur));
            io::stdout().flush().unwrap();
        } else {
            print!("          ");
        }

        if L::can_codegen() {
            if cfg!(feature = "parse-struct") {
                let dur = bench_parse_struct::<L>(contents.as_slice(), file);
                print!("{:6}.{}ms", millis(dur), tenths(dur));
                io::stdout().flush().unwrap();
            } else {
                print!("          ");
            }

            if cfg!(feature = "stringify-struct") {
                let parsed = L::parse_struct(contents.as_slice(), file);
                let dur = bench_stringify_struct::<L>(parsed, contents.len());
                print!("{:6}.{}ms", millis(dur), tenths(dur));
                io::stdout().flush().unwrap();
            }
        }

        println!("");
    }
}

fn bench_parse_dom<L: Library>(contents: &[u8]) -> time::Duration {
    timer::bench(num_trials(), || {
        L::parse_dom(contents).ok();
    })
}

fn bench_stringify_dom<L: Library>(dom: &L::DOM, len: usize) -> time::Duration {
    timer::bench_with_buf(num_trials(), len, |out| {
        L::stringify_dom(out, &dom)
    })
}

fn bench_parse_struct<L: Library>(contents: &[u8], which: TestFile) -> time::Duration {
    timer::bench(num_trials(), || {
        L::parse_struct(contents, which)
    })
}

fn bench_stringify_struct<L: Library>(contents: Contents, len: usize) -> time::Duration {
    timer::bench_with_buf(num_trials(), len, |out| {
        L::stringify_struct(out, &contents)
    })
}

fn millis(dur: time::Duration) -> i64 {
    dur.num_milliseconds()
}

fn tenths(dur: time::Duration) -> u8 {
    (dur.num_microseconds().unwrap() / 100 % 10) as u8
}

#[cfg(feature = "lib-serde")]
mod serde_json {
    use super::*;
    extern crate serde_json;

    pub struct Library;

    impl super::Library for Library {
        type DOM = serde_json::Value;

        fn name() -> &'static str { "serde_json" }

        fn parse_dom<R: io::Read>(mut read: R) -> Result<serde_json::Value, ()> {
            let mut string = String::new();
            read.read_to_string(&mut string).unwrap();
            serde_json::from_str(&string).or(Err(()))
        }

        fn stringify_dom<W: io::Write>(write: &mut W, dom: &serde_json::Value) {
            serde_json::to_writer(write, &dom).unwrap();
        }

        fn can_codegen() -> bool { true }

        fn parse_struct<R>(mut read: R, which: TestFile) -> Contents
            where R: io::Read,
        {
            let mut string = String::new();
            read.read_to_string(&mut string).unwrap();
            match which {
                #[cfg(feature = "file-canada")]
                TestFile::Canada => {
                    Contents::Canada(serde_json::from_str(&string).unwrap())
                }
                #[cfg(feature = "file-citm-catalog")]
                TestFile::CitmCatalog => {
                    Contents::CitmCatalog(serde_json::from_str(&string).unwrap())
                }
                #[cfg(feature = "file-twitter")]
                TestFile::Twitter => {
                    Contents::Twitter(serde_json::from_str(&string).unwrap())
                }
            }
        }

        #[cfg(any(feature = "file-canada",
                  feature = "file-citm-catalog",
                  feature = "file-twitter"))]
        fn stringify_struct<W>(write: &mut W, obj: &Contents)
            where W: io::Write,
        {
            match *obj {
                #[cfg(feature = "file-canada")]
                Contents::Canada(ref v) => serde_json::to_writer(write, v),
                #[cfg(feature = "file-citm-catalog")]
                Contents::CitmCatalog(ref v) => serde_json::to_writer(write, v),
                #[cfg(feature = "file-twitter")]
                Contents::Twitter(ref v) => serde_json::to_writer(write, v),
            }.unwrap();
        }
    }
}

#[cfg(feature = "lib-json-rust")]
mod json_rust {
    use super::*;
    extern crate json;

    use self::json::JsonValue;

    pub struct Library;

    impl super::Library for Library {
        type DOM = JsonValue;

        fn name() -> &'static str { "json-rust" }

        fn parse_dom<R: io::Read>(mut read: R) -> Result<JsonValue, ()> {
            let mut string = String::new();
            read.read_to_string(&mut string).unwrap();
            json::parse(&string).or(Err(()))
        }

        fn stringify_dom<W: io::Write>(write: &mut W, dom: &JsonValue) {
            dom.write(write).unwrap();
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
mod rustc {
    use super::*;

    use rustc_serialize::json;

    pub struct Library;

    impl super::Library for Library {
        type DOM = json::Json;

        fn name() -> &'static str { "rustc_serialize" }

        fn parse_dom<R: io::Read>(mut read: R) -> Result<json::Json, ()> {
            json::Json::from_reader(&mut read).or(Err(()))
        }

        fn stringify_dom<W: io::Write>(write: &mut W, dom: &json::Json) {
            use rustc_serialize::Encodable;
            let mut write = adapter::IoWriteAsFmtWrite::new(write);
            dom.encode(&mut json::Encoder::new(&mut write)).unwrap();
        }

        fn can_codegen() -> bool { true }

        fn parse_struct<R>(mut read: R, which: TestFile) -> Contents
            where R: io::Read,
        {
            let mut string = String::new();
            read.read_to_string(&mut string).unwrap();
            match which {
                #[cfg(feature = "file-canada")]
                TestFile::Canada => {
                    Contents::Canada(json::decode(&string).unwrap())
                }
                #[cfg(feature = "file-citm-catalog")]
                TestFile::CitmCatalog => {
                    Contents::CitmCatalog(json::decode(&string).unwrap())
                }
                #[cfg(feature = "file-twitter")]
                TestFile::Twitter => {
                    Contents::Twitter(json::decode(&string).unwrap())
                }
            }
        }

        #[cfg(any(feature = "file-canada",
                  feature = "file-citm-catalog",
                  feature = "file-twitter"))]
        fn stringify_struct<W>(write: &mut W, obj: &Contents)
            where W: io::Write,
        {
            use rustc_serialize::Encodable;
            let mut write = adapter::IoWriteAsFmtWrite::new(write);
            let mut encoder = json::Encoder::new(&mut write);
            match *obj {
                #[cfg(feature = "file-canada")]
                Contents::Canada(ref v) => v.encode(&mut encoder),
                #[cfg(feature = "file-citm-catalog")]
                Contents::CitmCatalog(ref v) => v.encode(&mut encoder),
                #[cfg(feature = "file-twitter")]
                Contents::Twitter(ref v) => v.encode(&mut encoder),
            }.unwrap();
        }
    }
}
