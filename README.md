# Rust JSON Benchmark

You can see the results of this benchmarks with some commentary at [json.rs](http://json.rs).

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 0.9.9
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.5
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.22

#### `$ cargo run --release --bin json-benchmark`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          14.2ms    11.7ms     6.3ms     8.2ms
data/citm_catalog.json     8.9ms     1.3ms     2.9ms     0.7ms
data/twitter.json          3.3ms     0.5ms     1.5ms     0.5ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json          11.0ms     3.1ms
data/citm_catalog.json     5.6ms     0.8ms
data/twitter.json          2.0ms     0.5ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          20.2ms    44.0ms    22.6ms    63.7ms
data/citm_catalog.json    16.3ms     3.6ms    18.9ms     2.8ms
data/twitter.json          8.3ms     1.8ms    10.5ms     1.7ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.17.0-nightly (413a975e3 2017-02-23)

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
