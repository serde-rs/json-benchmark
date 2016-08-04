# Rust JSON Benchmark

You can see the results of this benchmarks with some commentary at [json.rs](http://json.rs).

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 0.8.0
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.10.0
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.19

#### `$ cargo run --release`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          24.3ms    15.0ms    11.4ms     9.2ms
data/citm_catalog.json    16.3ms     2.4ms     6.1ms     1.2ms
data/twitter.json          6.3ms     0.9ms     3.2ms     0.8ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json          14.9ms     4.2ms
data/citm_catalog.json     8.7ms     1.5ms
data/twitter.json          3.4ms     0.9ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          32.5ms    55.1ms    38.6ms    77.6ms
data/citm_catalog.json    23.5ms     5.0ms    29.0ms     3.3ms
data/twitter.json         11.4ms     2.2ms    15.4ms     2.0ms
```

- Intel(R) Core(TM) i5-3210M CPU @ 2.50GHz
- rustc 1.12.0-nightly (a005b6785 2016-08-02)

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
