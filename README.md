# Rust JSON Benchmark

You can see the results of this benchmarks with some commentary at [json.rs](http://json.rs).

This is a partial port of
[nativejson-benchmark](https://github.com/miloyip/nativejson-benchmark)
to Rust. The libraries tested are:

- [serde\_json](https://github.com/serde-rs/json) 0.8.4
- [json-rust](https://github.com/maciejhirsz/json-rust) 0.10.3
- [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) 0.3.22

#### `$ cargo run --release`

```
                                DOM                STRUCT
======= serde_json ======= parse|stringify === parse|stringify ===
data/canada.json          14.0ms    11.5ms     6.9ms     6.9ms
data/citm_catalog.json     9.1ms     1.4ms     2.8ms     0.7ms
data/twitter.json          3.4ms     0.5ms     1.3ms     0.5ms

======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json           8.6ms     2.9ms
data/citm_catalog.json     4.8ms     0.7ms
data/twitter.json          1.8ms     0.5ms

==== rustc_serialize ===== parse|stringify === parse|stringify ===
data/canada.json          19.1ms    41.6ms    21.3ms    60.6ms
data/citm_catalog.json    14.9ms     3.5ms    17.6ms     2.7ms
data/twitter.json          7.6ms     1.6ms     9.7ms     1.5ms
```

- Intel(R) Core(TM) i7-6600U CPU @ 2.60GHz
- rustc 1.15.0-nightly (71c06a56a 2016-12-18)

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
