# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.2
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.6
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24

#### `$ cargo run --release --bin json-benchmark`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          10.2ms    11.1ms     4.2ms     6.9ms
data/citm_catalog.json     5.3ms     1.3ms     2.1ms     0.8ms
data/twitter.json          2.5ms     0.6ms     1.3ms     0.6ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json           8.4ms     2.9ms
data/citm_catalog.json     3.7ms     0.8ms
data/twitter.json          1.6ms     0.6ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          17.6ms    41.1ms    22.0ms    59.2ms
data/citm_catalog.json    13.9ms     3.6ms    17.9ms     2.9ms
data/twitter.json          7.8ms     1.8ms    10.2ms     1.7ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.19.0-nightly (f4209651e 2017-05-05)

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
