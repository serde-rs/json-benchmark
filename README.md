# Rust JSON Benchmark

This is a partial port of [nativejson-benchmark] to Rust. The libraries tested
are:

- [serde\_json] 1.0.53
- [ijson] 0.1.3
- [json-rust] 0.12.4
- [rustc-serialize] 0.3.24
- [simd-json] 0.1.26 (this requires a modern x86 CPU for good results)

[nativejson-benchmark]: https://github.com/miloyip/nativejson-benchmark
[serde\_json]: https://github.com/serde-rs/json
[serde\_json with ijson's IValue]: https://github.com/serde-rs/json + https://github.com/Diggsey/ijson
[json-rust]: https://github.com/maciejhirsz/json-rust
[rustc-serialize]: https://github.com/rust-lang-nursery/rustc-serialize
[simd-json]: https://github.com/Licenser/simdjson-rs

#### `$ cargo run --release`

```
                                DOM                  STRUCT
======= serde_json ======= parse|stringify ===== parse|stringify ====
data/canada.json         320 MB/s   490 MB/s   530 MB/s   370 MB/s
data/citm_catalog.json   410 MB/s   590 MB/s   760 MB/s   910 MB/s
data/twitter.json        290 MB/s   820 MB/s   590 MB/s  1070 MB/s

========= ijson ========== parse|stringify ===== parse|stringify ====
data/canada.json         260 MB/s   420 MB/s
data/citm_catalog.json   330 MB/s   560 MB/s
data/twitter.json        210 MB/s   870 MB/s

======= json-rust ======== parse|stringify ===== parse|stringify ====
data/canada.json         370 MB/s  1010 MB/s
data/citm_catalog.json   620 MB/s   900 MB/s
data/twitter.json        470 MB/s  1120 MB/s

==== rustc_serialize ===== parse|stringify ===== parse|stringify ====
data/canada.json         180 MB/s    78 MB/s   140 MB/s    53 MB/s
data/citm_catalog.json   220 MB/s   210 MB/s   160 MB/s   240 MB/s
data/twitter.json        130 MB/s   360 MB/s    94 MB/s   400 MB/s

======= simd-json ======== parse|stringify ===== parse|stringify ====
data/canada.json         400 MB/s   530 MB/s   670 MB/s
data/citm_catalog.json   910 MB/s   760 MB/s  1590 MB/s
data/twitter.json        890 MB/s   870 MB/s  1080 MB/s
```

- Intel(R) Xeon(R) Gold 6230 CPU @ 2.10GHz *(20-core x86 high performance server)*
- rustc 1.56.0-nightly (2827db2b1 2021-08-01)

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
