# Rust JSON Benchmark

This is a partial port of
[nativejson-benchark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 0.8.0-rc
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.8.4
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.19

#### `$ cargo run --release`

> ```
>      Running `target/release/json-benchmark`
>                                 DOM                STRUCT
> ======= serde_json ======= parse|stringify === parse|stringify ===
> data/canada.json          46.0ms    18.8ms    33.9ms    16.1ms
> data/citm_catalog.json    27.0ms     2.8ms    17.5ms     1.4ms
> data/twitter.json          9.2ms     1.3ms     6.9ms     1.3ms
>
> ======= json-rust ======== parse|stringify === parse|stringify ===
> data/canada.json          38.9ms    36.8ms
> data/citm_catalog.json    15.2ms     3.0ms
> data/twitter.json          6.1ms     2.0ms
>
> ==== rustc_serialize ===== parse|stringify === parse|stringify ===
> data/canada.json          34.2ms    54.0ms    40.2ms    50.7ms
> data/citm_catalog.json    23.5ms     5.4ms    28.7ms     3.5ms
> data/twitter.json         11.6ms     2.5ms    15.6ms     2.3ms
> ```

#### `$ cargo run --features conformance --no-default-features`

- serde\_json: 82%
- json-rust: 79%
- rustc-serialize: 82%
