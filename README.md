# Rust JSON Benchmark

This is a partial port of [nativejson-benchmark] to Rust. The libraries tested
are:

- [serde\_json] 1.0.53
- [json-rust] 0.12.4
- [rustc-serialize] 0.3.24
- [simd-json] 0.1.26 (this requires a modern x86 CPU for good results)

[nativejson-benchmark]: https://github.com/miloyip/nativejson-benchmark
[serde\_json]: https://github.com/serde-rs/json
[json-rust]: https://github.com/maciejhirsz/json-rust
[rustc-serialize]: https://github.com/rust-lang-nursery/rustc-serialize
[simd-json]: https://github.com/Licenser/simdjson-rs

#### `$ cargo run --release`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify ===== parse|stringify ====
data/canada.json         320 MB/s   430 MB/s   580 MB/s   310 MB/s
data/citm_catalog.json   420 MB/s   560 MB/s   710 MB/s   880 MB/s
data/twitter.json        300 MB/s   910 MB/s   550 MB/s  1060 MB/s

======= json-rust ======== parse|stringify ===== parse|stringify ====
data/canada.json         390 MB/s   840 MB/s
data/citm_catalog.json   520 MB/s   780 MB/s
data/twitter.json        430 MB/s  1030 MB/s

==== rustc_serialize ===== parse|stringify ===== parse|stringify ====
data/canada.json         150 MB/s    67 MB/s   120 MB/s    46 MB/s
data/citm_catalog.json   210 MB/s   180 MB/s   140 MB/s   210 MB/s
data/twitter.json        120 MB/s   330 MB/s    87 MB/s   350 MB/s

======= simd-json ======== parse|stringify ===== parse|stringify ====
data/canada.json         370 MB/s   440 MB/s   600 MB/s
data/citm_catalog.json   920 MB/s   670 MB/s  1400 MB/s
data/twitter.json        790 MB/s   880 MB/s   980 MB/s
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz *(laptop CPU from 2015)*
- rustc 1.59.0-nightly (0b6f079e4 2021-12-07)

To update the numbers above, I run `./json-benchmark` twice on an otherwise idle
computer and take the greater of the two results for each number.

For comparison, here are results from [RapidJSON] on the same hardware with the
nativejson-benchmark modified to run 256 times instead of 10. Code is in the
`cpp` directory of this repo.

[RapidJSON]: https://github.com/miloyip/rapidjson

```
==== rapidjson-clang ===================== parse|stringify ====
data/canada.json                         380 MB/s   200 MB/s
data/citm_catalog.json                   680 MB/s  1090 MB/s
data/twitter.json                        320 MB/s   600 MB/s

===== rapidjson-gcc ====================== parse|stringify ====
data/canada.json                         390 MB/s   250 MB/s
data/citm_catalog.json                   890 MB/s  1650 MB/s
data/twitter.json                        440 MB/s   870 MB/s
```

- clang version 11.0.0-++20200128080810+b96e6859c99-1~exp1~20200128191406.2758
- gcc version 10.1.0 (Ubuntu 10.1.0-2ubuntu1~18.04)

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
