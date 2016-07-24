# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 0.8.0-rc
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.9.1-rc
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.19

#### `$ cargo run --release`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          24.7ms    14.8ms    11.7ms     9.5ms
data/citm_catalog.json    16.6ms     2.4ms     6.4ms     1.2ms
data/twitter.json          6.4ms     1.0ms     3.7ms     0.9ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json          14.8ms     4.5ms
data/citm_catalog.json     9.0ms     1.3ms
data/twitter.json          3.4ms     0.8ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          33.4ms    53.2ms    41.4ms    74.1ms
data/citm_catalog.json    23.7ms     5.2ms    29.6ms     3.6ms
data/twitter.json         11.6ms     2.4ms    15.9ms     2.2ms
```

- Intel(R) Core(TM) i5-3210M CPU @ 2.50GHz
- rustc 1.12.0-nightly (936bfea94 2016-07-20)

To update the numbers above, I run `./json-benchmark -n 256` twice on an
otherwise idle computer and take the least of the two results for each number.

For comparison, here are results from
[RapidJSON](https://github.com/miloyip/rapidjson) on the same hardware with the
nativejson-benchmark modified to run 256 times instead of 10.

```
                                DOM
======= rapidjson ======== parse|stringify ===
data/canada.json           5.4ms     9.8ms
data/citm_catalog.json     2.2ms     1.3ms
data/twitter.json          1.0ms     1.0ms
```
