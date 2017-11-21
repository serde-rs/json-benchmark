# Rust JSON Benchmark

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 1.0.6
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.11.12
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.24

#### `$ cargo run --release --bin json-benchmark`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json           9.9ms     9.4ms     4.4ms     7.0ms
data/citm_catalog.json     5.4ms     1.3ms     2.0ms     0.7ms
data/twitter.json          2.6ms     0.6ms     1.2ms     0.6ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json           6.3ms     3.4ms
data/citm_catalog.json     3.2ms     0.9ms
data/twitter.json          1.4ms     0.6ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          15.8ms    40.0ms    20.0ms    54.3ms
data/citm_catalog.json    11.5ms     3.4ms    15.5ms     2.8ms
data/twitter.json          6.1ms     1.7ms     8.5ms     1.5ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.23.0-nightly (33374fa9d 2017-11-20)

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
