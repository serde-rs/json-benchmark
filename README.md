# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.45
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.12.1
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24
- [simd-json](https://github.com/Licenser/simdjson-rs) 0.1.26 (this requires a modern x86 CPU for good results)

#### `$ cargo run --release`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify ===== parse|stringify ====
data/canada.json         220 MB/s   410 MB/s   490 MB/s   320 MB/s
data/citm_catalog.json   370 MB/s   460 MB/s   820 MB/s   750 MB/s
data/twitter.json        250 MB/s   820 MB/s   550 MB/s   930 MB/s

======= json-rust ======== parse|stringify ===== parse|stringify ====
data/canada.json         390 MB/s   840 MB/s
data/citm_catalog.json   560 MB/s   700 MB/s
data/twitter.json        420 MB/s   840 MB/s

==== rustc_serialize ===== parse|stringify ===== parse|stringify ====
data/canada.json         140 MB/s    57 MB/s   100 MB/s    38 MB/s
data/citm_catalog.json   160 MB/s   150 MB/s   110 MB/s   210 MB/s
data/twitter.json         86 MB/s   330 MB/s    67 MB/s   360 MB/s

======= simd-json ======== parse|stringify ===== parse|stringify ====
data/canada.json         310 MB/s   420 MB/s   570 MB/s
data/citm_catalog.json   840 MB/s   580 MB/s  1330 MB/s
data/twitter.json        780 MB/s   720 MB/s   930 MB/s
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz *(laptop CPU from 2015)*
- rustc 1.42.0-nightly (13db6501c 2020-02-01)

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
