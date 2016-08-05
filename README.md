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
data/canada.json          19.9ms    13.8ms    10.1ms     9.2ms
data/citm_catalog.json    13.8ms     2.5ms     6.0ms     1.2ms
data/twitter.json          5.6ms     0.9ms     2.9ms     0.8ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json          13.5ms     4.1ms
data/citm_catalog.json     8.5ms     1.3ms
data/twitter.json          3.1ms     0.8ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          29.6ms    55.8ms    32.8ms    78.1ms
data/citm_catalog.json    22.6ms     5.1ms    26.4ms     3.4ms
data/twitter.json         11.3ms     2.2ms    14.5ms     2.2ms
```

- Intel(R) Core(TM) i5-3210M CPU @ 2.50GHz
- rustc 1.12.0-nightly (0a3180baa 2016-08-03)

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
