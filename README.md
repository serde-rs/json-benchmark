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
data/canada.json          15.6ms    11.5ms     8.3ms     6.9ms
data/citm_catalog.json    11.1ms     1.6ms     4.2ms     0.8ms
data/twitter.json          4.2ms     0.6ms     1.8ms     0.5ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json          10.1ms     3.2ms
data/citm_catalog.json     6.3ms     0.8ms
data/twitter.json          2.3ms     0.6ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          21.4ms    41.9ms    23.6ms    61.5ms
data/citm_catalog.json    17.5ms     4.5ms    20.1ms     2.6ms
data/twitter.json          8.5ms     1.7ms    10.7ms     1.5ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.13.0-nightly (3c5a0fa45 2016-08-22)

To update the numbers above, I run `./json-benchmark -n 256` twice on an
otherwise idle computer and take the least of the two results for each number.

For comparison, here are results from
[RapidJSON](https://github.com/miloyip/rapidjson) on the same hardware with the
nativejson-benchmark modified to run 256 times instead of 10.

```
                                DOM
==== rapidjson-clang ===== parse|stringify ===
data/canada.json           4.6ms     9.5ms
data/citm_catalog.json     2.0ms     1.1ms
data/twitter.json          1.4ms     0.9ms

===== rapidjson-gcc ====== parse|stringify ===
data/canada.json           3.9ms     7.0ms
data/citm_catalog.json     1.7ms     0.9ms
data/twitter.json          1.3ms     0.9ms
```

- clang version 3.8.0
- gcc version 5.4.0
