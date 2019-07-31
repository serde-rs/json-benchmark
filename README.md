# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.39
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.14
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24
- [simd-json](https://github.com/Licenser/simdjson-rs) 0.1.15 (this requires a modern x86 CPU for good results)

#### `$ cargo run --release`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify ===== parse|stringify ====
data/canada.json         230 MB/s   420 MB/s   530 MB/s   330 MB/s
data/citm_catalog.json   390 MB/s   450 MB/s   860 MB/s   810 MB/s
data/twitter.json        260 MB/s   870 MB/s   570 MB/s   940 MB/s

======= json-rust ======== parse|stringify ===== parse|stringify ====
data/canada.json         360 MB/s   920 MB/s
data/citm_catalog.json   520 MB/s   770 MB/s
data/twitter.json        420 MB/s   900 MB/s

==== rustc_serialize ===== parse|stringify ===== parse|stringify ====
data/canada.json         140 MB/s    56 MB/s    96 MB/s    42 MB/s
data/citm_catalog.json   150 MB/s   150 MB/s   110 MB/s   190 MB/s
data/twitter.json         83 MB/s   290 MB/s    64 MB/s   310 MB/s

======= simd-json ======== parse|stringify ===== parse|stringify ====
data/canada.json         340 MB/s   460 MB/s   570 MB/s
data/citm_catalog.json   780 MB/s   630 MB/s  1230 MB/s
data/twitter.json        670 MB/s   990 MB/s   800 MB/s
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz *(laptop CPU from 2015)*
- rustc 1.37.0-nightly (d3e2cec29 2019-06-26)

To update the numbers above, I run `./json-benchmark` twice on an otherwise idle
computer and take the greater of the two results for each number.

For comparison, here are results from
[RapidJSON](https://github.com/miloyip/rapidjson) on the same hardware with the
nativejson-benchmark modified to run 256 times instead of 10.

```
==== rapidjson-clang ===================== parse|stringify ====
data/canada.json                         390 MB/s   200 MB/s
data/citm_catalog.json                   670 MB/s   290 MB/s
data/twitter.json                        340 MB/s   370 MB/s

===== rapidjson-gcc ====================== parse|stringify ====
data/canada.json                         470 MB/s   240 MB/s
data/citm_catalog.json                   990 MB/s   480 MB/s
data/twitter.json                        470 MB/s   620 MB/s
```

- clang version 3.8.0
- gcc version 5.4.0

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
