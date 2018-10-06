# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.32
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.13
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24

#### `$ cargo run --release --bin json-benchmark`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify ===== parse|stringify ====
data/canada.json         210 MB/s   370 MB/s   540 MB/s   300 MB/s
data/citm_catalog.json   290 MB/s   430 MB/s   840 MB/s   780 MB/s
data/twitter.json        250 MB/s   860 MB/s   550 MB/s   930 MB/s

======= json-rust ======== parse|stringify =====  not supported  ====
data/canada.json         330 MB/s   720 MB/s
data/citm_catalog.json   500 MB/s   740 MB/s
data/twitter.json        450 MB/s   890 MB/s

==== rustc_serialize ===== parse|stringify ===== parse|stringify ====
data/canada.json         130 MB/s    55 MB/s    97 MB/s    41 MB/s
data/citm_catalog.json   130 MB/s   130 MB/s    87 MB/s   170 MB/s
data/twitter.json         79 MB/s   290 MB/s    61 MB/s   310 MB/s
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz *(laptop CPU from 2015)*
- rustc 1.31.0-nightly (8c4ad4e9e 2018-10-04)

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
