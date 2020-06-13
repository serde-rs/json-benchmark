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
data/canada.json         280 MB/s   370 MB/s   510 MB/s   320 MB/s
data/citm_catalog.json   400 MB/s   420 MB/s   840 MB/s   660 MB/s
data/twitter.json        270 MB/s   730 MB/s   530 MB/s   850 MB/s

======= json-rust ======== parse|stringify ===== parse|stringify ====
data/canada.json         270 MB/s   830 MB/s
data/citm_catalog.json   550 MB/s   700 MB/s
data/twitter.json        410 MB/s   900 MB/s

==== rustc_serialize ===== parse|stringify ===== parse|stringify ====
data/canada.json         150 MB/s    65 MB/s   110 MB/s    45 MB/s
data/citm_catalog.json   180 MB/s   180 MB/s   130 MB/s   210 MB/s
data/twitter.json         99 MB/s   320 MB/s    75 MB/s   350 MB/s

======= simd-json ======== parse|stringify ===== parse|stringify ====
data/canada.json         350 MB/s   420 MB/s   580 MB/s
data/citm_catalog.json   820 MB/s   590 MB/s  1290 MB/s
data/twitter.json        660 MB/s   740 MB/s   900 MB/s
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz *(laptop CPU from 2015)*
- rustc 1.46.0-nightly (118b50524 2020-06-06)

To update the numbers above, I run `./json-benchmark` twice on an otherwise idle
computer and take the greater of the two results for each number.

For comparison, here are results from [RapidJSON] on the same hardware with the
nativejson-benchmark modified to run 256 times instead of 10. Code is in the
`cpp` directory of this repo.

[RapidJSON]: https://github.com/miloyip/rapidjson

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
