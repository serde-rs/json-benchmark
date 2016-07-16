# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 0.8.0-rc
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.9.0-rc
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.19

#### `$ cargo run --release`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          23.7ms    16.9ms    11.1ms    15.2ms
data/citm_catalog.json    16.3ms     2.4ms     6.1ms     1.2ms
data/twitter.json          6.6ms     0.9ms     3.6ms     0.9ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json          15.6ms    14.7ms
data/citm_catalog.json     9.0ms     1.4ms
data/twitter.json          3.3ms     0.9ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          34.2ms    54.0ms    40.2ms    50.7ms
data/citm_catalog.json    23.5ms     5.4ms    28.7ms     3.5ms
data/twitter.json         11.6ms     2.5ms    15.6ms     2.3ms
```

- Intel(R) Core(TM) i5-3210M CPU @ 2.50GHz
- rustc 1.11.0-nightly (ad7fe6521 2016-06-23)
