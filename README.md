# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.35
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.13
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24

#### `$ cargo run --release --bin json-benchmark`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify ===== parse|stringify ====
data/canada.json         200 MB/s   390 MB/s   550 MB/s   320 MB/s
data/citm_catalog.json   290 MB/s   370 MB/s   860 MB/s   790 MB/s
data/twitter.json        260 MB/s   850 MB/s   550 MB/s   940 MB/s

======= json-rust ======== parse|stringify ===== parse|stringify ====
data/canada.json         270 MB/s   830 MB/s
data/citm_catalog.json   560 MB/s   660 MB/s
data/twitter.json        420 MB/s   870 MB/s

==== rustc_serialize ===== parse|stringify ===== parse|stringify ====
data/canada.json         130 MB/s    60 MB/s   100 MB/s    45 MB/s
data/citm_catalog.json   130 MB/s   140 MB/s    97 MB/s   200 MB/s
data/twitter.json         92 MB/s   330 MB/s    69 MB/s   350 MB/s
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz *(laptop CPU from 2015)*
- rustc 1.33.0-nightly (75a369c5b 2019-01-12)

To update the numbers above, I run `./json-benchmark -n 256` twice on an
otherwise idle computer and take the greater of the two results for each number.

For comparison, here are results from
[RapidJSON](https://github.com/miloyip/rapidjson) on the same hardware with the
nativejson-benchmark modified to run 256 times instead of 10.

```
                                DOM
==== rapidjson-clang ===== parse|stringify ====
data/canada.json         390 MB/s   200 MB/s
data/citm_catalog.json   670 MB/s   290 MB/s
data/twitter.json        340 MB/s   370 MB/s

===== rapidjson-gcc ====== parse|stringify ====
data/canada.json         470 MB/s   240 MB/s
data/citm_catalog.json   990 MB/s   480 MB/s
data/twitter.json        470 MB/s   620 MB/s
```

- clang version 3.8.0
- gcc version 5.4.0
