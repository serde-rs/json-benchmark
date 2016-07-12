# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 0.8.0-rc
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.8.8
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.19

#### `$ cargo run --release`

> ```
>                                 DOM                STRUCT
> ======= serde_json ======= parse|stringify === parse|stringify ===
> data/canada.json          24.3ms    19.4ms    11.1ms    16.0ms
> data/citm_catalog.json    16.9ms     2.9ms     6.2ms     1.3ms
> data/twitter.json          6.5ms     1.0ms     4.1ms     1.1ms
>
> ======= json-rust ======== parse|stringify === parse|stringify ===
> data/canada.json          32.8ms    19.6ms
> data/citm_catalog.json    15.1ms     1.8ms
> data/twitter.json          6.0ms     0.9ms
>
> ==== rustc_serialize ===== parse|stringify === parse|stringify ===
> data/canada.json          34.2ms    54.0ms    40.2ms    50.7ms
> data/citm_catalog.json    23.5ms     5.4ms    28.7ms     3.5ms
> data/twitter.json         11.6ms     2.5ms    15.6ms     2.3ms
> ```

- Intel(R) Core(TM) i5-3210M CPU @ 2.50GHz
- rustc 1.11.0-nightly (ad7fe6521 2016-06-23)

#### `$ cargo run --features conformance --no-default-features`

- serde\_json: 90%
- json-rust: 84%
- rustc-serialize: 82%
