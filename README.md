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
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          10.5ms     5.5ms     4.1ms     4.0ms
data/citm_catalog.json     5.9ms     1.0ms     2.0ms     0.6ms
data/twitter.json          2.5ms     0.5ms     1.1ms     0.5ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json           6.6ms     3.1ms
data/citm_catalog.json     3.5ms     0.6ms
data/twitter.json          1.3ms     0.5ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          17.4ms    39.6ms    24.9ms    53.3ms
data/citm_catalog.json    14.3ms     3.6ms    19.0ms     2.9ms
data/twitter.json          7.8ms     1.5ms    10.2ms     1.5ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.31.0-nightly (8c4ad4e9e 2018-10-04)

To update the numbers above, I run `./json-benchmark -n 256` twice on an
otherwise idle computer and take the least of the two results for each number.

For comparison, here are results from
[RapidJSON](https://github.com/miloyip/rapidjson) on the same hardware with the
nativejson-benchmark modified to run 256 times instead of 10.

```
                                DOM
==== rapidjson-clang ===== parse|stringify ===
data/canada.json           5.7ms    10.5ms
data/citm_catalog.json     2.5ms     1.7ms
data/twitter.json          1.8ms     1.2ms

===== rapidjson-gcc ====== parse|stringify ===
data/canada.json           4.7ms     8.6ms
data/citm_catalog.json     1.7ms     1.0ms
data/twitter.json          1.3ms     0.7ms
```

- clang version 3.8.0
- gcc version 5.4.0

As another point of comparison, here are the same three json files encoded into
a compact binary format with Serde. To reproduce these numbers run `cargo run
--release --bin binary-benchmark --features binary`.

```
                                                   STRUCT
============================================== decode | encode ===
data/canada.json                              0.29ms    0.46ms
data/citm_catalog.json                        0.29ms    0.10ms
data/twitter.json                             0.35ms    0.09ms
```
