# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.26
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.13
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24

#### `$ cargo run --release --bin json-benchmark`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          10.2ms     5.4ms     3.9ms     4.0ms
data/citm_catalog.json     5.7ms     1.0ms     1.9ms     0.6ms
data/twitter.json          2.4ms     0.5ms     1.1ms     0.5ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json           7.0ms     3.1ms
data/citm_catalog.json     3.4ms     0.6ms
data/twitter.json          1.4ms     0.5ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          17.2ms    40.3ms    24.9ms    57.5ms
data/citm_catalog.json    14.1ms     4.1ms    18.6ms     3.2ms
data/twitter.json          7.8ms     1.9ms    10.1ms     1.8ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.30.0-nightly (73c78734b 2018-08-05)

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
