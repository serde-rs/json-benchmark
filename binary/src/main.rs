use json_benchmark::*;

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

macro_rules! bench_file {
    {
        path: $path:expr,
        structure: $structure:ty,
    } => {
        let num_trials = num_trials().unwrap_or(4096);

        print!("{:27}", $path);
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
            let len = binary.len();
            let dur = timer::bench(num_trials, || {
                let parsed: $structure = serde_bench::deserialize(&binary).unwrap();
                parsed
            });
            print!("{:7} MB/s", throughput(dur, len));
            io::stdout().flush().unwrap();
        }
        #[cfg(not(feature = "parse-struct"))]
        print!("           ");

        #[cfg(feature = "stringify-struct")]
        {
            let len = binary.len();
            let dur = timer::bench_with_buf(num_trials, len, |out| {
                serde_bench::serialize(out, &data).unwrap()
            });
            print!("{:7} MB/s", throughput(dur, len));
            io::stdout().flush().unwrap();
        }

        println!("");
    }
}

fn main() {
    env::set_current_dir("..").unwrap();

    println!("{:>45}", "STRUCT");
    println!("================================= decode | encode ===");

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
