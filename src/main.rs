#![feature(stmt_expr_attributes)]

#![cfg_attr(feature = "lib-serde", feature(custom_derive, plugin))]
#![cfg_attr(feature = "lib-serde", plugin(serde_macros))]

extern crate dtoa;
extern crate fastwrite;
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

use std::{env, mem};
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

    fn parse_double(j: &str) -> Result<f64, ()>;
    fn parse_string(j: &str) -> Result<String, ()>;
}

fn main() {
    if cfg!(any(feature = "performance",
                feature = "parse-dom",
                feature = "stringify-dom",
                feature = "parse-struct",
                feature = "stringify-struct")) {
        print!("{:>35}{:>22}", "DOM", "STRUCT");
        #[cfg(feature = "lib-serde")]
        bench::<serde_json::Library>();
        #[cfg(feature = "lib-json-rust")]
        bench::<json_rust::Library>();
        #[cfg(feature = "lib-rustc-serialize")]
        bench::<rustc::Library>();
    }

    if cfg!(feature = "conformance") {
        #[cfg(feature = "lib-serde")]
        conformance::<serde_json::Library>();
        #[cfg(feature = "lib-json-rust")]
        conformance::<json_rust::Library>();
        #[cfg(feature = "lib-rustc-serialize")]
        conformance::<rustc::Library>();
    }
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

fn conformance<L: Library>() {
    println!("Testing {} conformance", L::name());
    conformance_failpass::<L>();
    conformance_doubles::<L>();
    conformance_strings::<L>();
    conformance_roundtrip::<L>();
}

fn conformance_failpass<L: Library>() {
    let mut pass = 0;
    let mut fail = 0;
    for i in 1..34 {
        // nativejson-benchmark skips these
        if i == 1 || i == 18 {
            continue;
        }
        let path = format!("data/jsonchecker/fail{:02}.json", i);
        let mut contents = Vec::new();
        File::open(&path).unwrap().read_to_end(&mut contents).unwrap();
        let dom = L::parse_dom(contents.as_slice());
        if dom.is_err() {
            pass += 1;
        } else {
            println!(" - failed {}", path);
            fail += 1;
        }
    }
    for i in 1..4 {
        let path = format!("data/jsonchecker/pass{:02}.json", i);
        let mut contents = Vec::new();
        File::open(&path).unwrap().read_to_end(&mut contents).unwrap();
        let dom = L::parse_dom(contents.as_slice());
        if dom.is_ok() {
            pass += 1;
        } else {
            println!(" - failed {}", path);
            fail += 1;
        }
    }
    println!(" - FAILPASS: {} / {}", pass, pass + fail);
}

fn conformance_doubles<L: Library>() {
    let mut pass = 0;
    let mut fail = 0;
    for &(json, expected) in [
        ("[0.0]", 0.0),
        ("[-0.0]", -0.0),
        ("[1.0]", 1.0),
        ("[-1.0]", -1.0),
        ("[1.5]", 1.5),
        ("[-1.5]", -1.5),
        ("[3.1416]", 3.1416),
        ("[1E10]", 1E10),
        ("[1e10]", 1e10),
        ("[1E+10]", 1E+10),
        ("[1E-10]", 1E-10),
        ("[-1E10]", -1E10),
        ("[-1e10]", -1e10),
        ("[-1E+10]", -1E+10),
        ("[-1E-10]", -1E-10),
        ("[1.234E+10]", 1.234E+10),
        ("[1.234E-10]", 1.234E-10),
        ("[1.79769e+308]", 1.79769e+308),
        ("[2.22507e-308]", 2.22507e-308),
        ("[-1.79769e+308]", -1.79769e+308),
        ("[-2.22507e-308]", -2.22507e-308),
        ("[4.9406564584124654e-324]", 4.9406564584124654e-324), // minimum denormal
        ("[2.2250738585072009e-308]", 2.2250738585072009e-308), // Max subnormal double
        ("[2.2250738585072014e-308]", 2.2250738585072014e-308), // Min normal positive double
        ("[1.7976931348623157e+308]", 1.7976931348623157e+308), // Max double
        ("[1e-10000]", 0.0),                                   // must underflow
        ("[18446744073709551616]", 18446744073709551616.0),    // 2^64 (max of uint64_t + 1, force to use double)
        ("[-9223372036854775809]", -9223372036854775809.0),    // -2^63 - 1(min of int64_t + 1, force to use double)
        ("[0.9868011474609375]", 0.9868011474609375),          // https://github.com/miloyip/rapidjson/issues/120
        ("[123e34]", 123e34),                                  // Fast Path Cases In Disguise
        ("[45913141877270640000.0]", 45913141877270640000.0),
        ("[2.2250738585072011e-308]", 2.2250738585072011e-308), // http://www.exploringbinary.com/php-hangs-on-numeric-value-2-2250738585072011e-308/
        ("[1e-214748363]", 0.0),
        ("[1e-214748364]", 0.0),
        ("[0.017976931348623157e+310]", 1.7976931348623157e+308), // Max double in another form

        // Since
        // abs((2^-1022 - 2^-1074) - 2.2250738585072012e-308) = 3.109754131239141401123495768877590405345064751974375599... ¡Á 10^-324
        // abs((2^-1022) - 2.2250738585072012e-308) = 1.830902327173324040642192159804623318305533274168872044... ¡Á 10 ^ -324
        // So 2.2250738585072012e-308 should round to 2^-1022 = 2.2250738585072014e-308
        ("[2.2250738585072012e-308]", 2.2250738585072014e-308), // http://www.exploringbinary.com/java-hangs-when-converting-2-2250738585072012e-308/

        // More closer to normal/subnormal boundary
        // boundary = 2^-1022 - 2^-1075 = 2.225073858507201136057409796709131975934819546351645648... ¡Á 10^-308
        ("[2.22507385850720113605740979670913197593481954635164564e-308]", 2.2250738585072009e-308),
        ("[2.22507385850720113605740979670913197593481954635164565e-308]", 2.2250738585072014e-308),

        // 1.0 is in (1.0 - 2^-54, 1.0 + 2^-53)
        // 1.0 - 2^-54 = 0.999999999999999944488848768742172978818416595458984375
        ("[0.999999999999999944488848768742172978818416595458984375]", 1.0), // round to even
        ("[0.999999999999999944488848768742172978818416595458984374]", 0.99999999999999989), // previous double
        ("[0.999999999999999944488848768742172978818416595458984376]", 1.0), // next double
        // 1.0 + 2^-53 = 1.00000000000000011102230246251565404236316680908203125
        ("[1.00000000000000011102230246251565404236316680908203125]", 1.0), // round to even
        ("[1.00000000000000011102230246251565404236316680908203124]", 1.0), // previous double
        ("[1.00000000000000011102230246251565404236316680908203126]", 1.00000000000000022), // next double

        // Numbers from https://github.com/floitsch/double-conversion/blob/master/test/cctest/test-strtod.cc

        ("[72057594037927928.0]", 72057594037927928.0),
        ("[72057594037927936.0]", 72057594037927936.0),
        ("[72057594037927932.0]", 72057594037927936.0),
        ("[7205759403792793199999e-5]", 72057594037927928.0),
        ("[7205759403792793200001e-5]", 72057594037927936.0),

        ("[9223372036854774784.0]", 9223372036854774784.0),
        ("[9223372036854775808.0]", 9223372036854775808.0),
        ("[9223372036854775296.0]", 9223372036854775808.0),
        ("[922337203685477529599999e-5]", 9223372036854774784.0),
        ("[922337203685477529600001e-5]", 9223372036854775808.0),

        ("[10141204801825834086073718800384]", 10141204801825834086073718800384.0),
        ("[10141204801825835211973625643008]", 10141204801825835211973625643008.0),
        ("[10141204801825834649023672221696]", 10141204801825835211973625643008.0),
        ("[1014120480182583464902367222169599999e-5]", 10141204801825834086073718800384.0),
        ("[1014120480182583464902367222169600001e-5]", 10141204801825835211973625643008.0),

        ("[5708990770823838890407843763683279797179383808]", 5708990770823838890407843763683279797179383808.0),
        ("[5708990770823839524233143877797980545530986496]", 5708990770823839524233143877797980545530986496.0),
        ("[5708990770823839207320493820740630171355185152]", 5708990770823839524233143877797980545530986496.0),
        ("[5708990770823839207320493820740630171355185151999e-3]", 5708990770823838890407843763683279797179383808.0),
        ("[5708990770823839207320493820740630171355185152001e-3]", 5708990770823839524233143877797980545530986496.0),

        ("[1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
           0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
           0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
           000000000]", 1e308),

        ("[2.22507385850720113605740979670913197593481954635164564802342610972482222202107694551652952390813508\
           7914149158913039621106870086438694594645527657207407820621743379988141063267329253552286881372149012\
           9811224514518898490572223072852551331557550159143974763979834118019993239625482890171070818506906306\
           6665599493827577257201576306269066333264756530000924588831643303777979186961204949739037782970490505\
           1080609940730262937128958950003583799967207254304360284078895771796150945516748243471030702609144621\
           5722898802581825451803257070188608721131280795122334262883686223215037756666225039825343359745688844\
           2390026549819838548794829220689472168983109969836584681402285424333066033985088644580400103493397042\
           7567186443383770486037861622771738545623065874679014086723327636718751234567890123456789012345678901\
           e-308]", 2.2250738585072014e-308),
    ].iter() {
        match L::parse_double(json) {
            Ok(v) if unsafe { mem::transmute::<_, u64>(v) == mem::transmute::<_, u64>(expected) } => {
                pass += 1;
            }
            Ok(v) => {
                println!(" - failed {}, expected {}, got {}", json, double(expected), double(v));
                fail += 1;
            }
            Err(_) => {
                println!(" - failed {}", json);
                fail += 1;
            }
        }
    }
    println!(" - DOUBLES: {} / {}", pass, pass + fail);
}

fn double(f: f64) -> String {
    let mut buf = Vec::new();
    dtoa::write(&mut buf, f).unwrap();
    String::from_utf8(buf).unwrap()
}

fn conformance_strings<L: Library>() {
    let mut pass = 0;
    let mut fail = 0;
    for &(json, expected) in [
        ("[\"\"]", ""),
        ("[\"Hello\"]", "Hello"),
        ("[\"Hello\\nWorld\"]", "Hello\nWorld"),
        ("[\"Hello\\u0000World\"]", "Hello\0World"),
        ("[\"\\\"\\\\/\\b\\f\\n\\r\\t\"]", "\"\\/\x08\x0C\n\r\t"),
        ("[\"\\u0024\"]", "\x24"),             // Dollar sign U+0024
        ("[\"\\u00A2\"]", "\u{A2}"),           // Cents sign U+00A2
        ("[\"\\u20AC\"]", "\u{20AC}"),         // Euro sign U+20AC
        ("[\"\\uD834\\uDD1E\"]", "\u{1D11E}"), // G clef sign U+1D11E
    ].iter() {
        match L::parse_string(json) {
            Ok(ref v) if v == expected => {
                pass += 1;
            }
            _ => {
                println!(" - failed {}", json);
                fail += 1;
            }
        }
    }
    println!(" - STRINGS: {} / {}", pass, pass + fail);
}

fn conformance_roundtrip<L: Library>() {
    let mut pass = 0;
    let mut fail = 0;
    for i in 1..28 {
        let path = format!("data/roundtrip/roundtrip{:02}.json", i);
        let mut contents = Vec::new();
        File::open(&path).unwrap().read_to_end(&mut contents).unwrap();
        let dom = L::parse_dom(contents.as_slice());
        match dom {
            Err(_) => {
                fail += 1;
            }
            Ok(dom) => {
                let mut out = Vec::new();
                L::stringify_dom(&mut out, &dom);
                if out == contents {
                    pass += 1;
                } else {
                    println!(" - failed {}, got {}", path, String::from_utf8(out).unwrap());
                    fail += 1;
                }
            }
        }
    }
    println!(" - ROUNDTRIP: {} / {}", pass, pass + fail);
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

        fn parse_double(j: &str) -> Result<f64, ()> {
            serde_json::from_str::<Vec<f64>>(j).map(|v| v[0]).or(Err(()))
        }

        fn parse_string(j: &str) -> Result<String, ()> {
            serde_json::from_str::<Vec<String>>(j).map(|v| v[0].clone()).or(Err(()))
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
            dom.to_writer(write);
        }

        fn parse_double(j: &str) -> Result<f64, ()> {
            json::parse(j).map(|v| v[0].as_f64().unwrap()).or(Err(()))
        }

        fn parse_string(j: &str) -> Result<String, ()> {
            json::parse(j).map(|v| v[0].as_str().unwrap().to_string()).or(Err(()))
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

        fn parse_double(j: &str) -> Result<f64, ()> {
            json::decode::<Vec<f64>>(j).map(|v| v[0]).or(Err(()))
        }

        fn parse_string(j: &str) -> Result<String, ()> {
            json::decode::<Vec<String>>(j).map(|v| v[0].clone()).or(Err(()))
        }
    }
}
