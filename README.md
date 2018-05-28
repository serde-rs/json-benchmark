# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.18
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.13
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24

#### `$ cargo run --release --bin json-benchmark`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          11.3ms     9.2ms     4.0ms     6.8ms
data/citm_catalog.json     6.3ms     1.3ms     2.0ms     0.8ms
data/twitter.json          2.5ms     0.7ms     1.1ms     0.6ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json           7.0ms     3.2ms
data/citm_catalog.json     3.4ms     0.9ms
data/twitter.json          1.4ms     0.6ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          17.8ms    41.1ms    23.4ms    55.7ms
data/citm_catalog.json    14.1ms     4.1ms    17.9ms     2.9ms
data/twitter.json          8.3ms     1.7ms    10.5ms     1.6ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.28.0-nightly (990d8aa74 2018-05-25)

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
