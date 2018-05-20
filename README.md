# Serde

```console
$ cargo bench

test deserialize_large  ... bench:      45,274 ns/iter (+/- 2,824)
test deserialize_medium ... bench:       2,940 ns/iter (+/- 180)
test deserialize_small  ... bench:         447 ns/iter (+/- 9)
test serialize_large    ... bench:      11,343 ns/iter (+/- 244)
test serialize_medium   ... bench:         723 ns/iter (+/- 47)
test serialize_small    ... bench:         360 ns/iter (+/- 22)
```

# GoJay

```console
$ cd $GOPATH/src/github.com/francoispqt/gojay/benchmarks/decoder && make bench

BenchmarkGoJayDecodeObjLarge-4           52713 ns/op
BenchmarkGoJayUnsafeDecodeObjLarge-4     46322 ns/op
BenchmarkGoJayDecodeObjMedium-4           4708 ns/op
BenchmarkGoJayUnsafeDecodeObjMedium-4     4096 ns/op
BenchmarkGoJayDecodeObjSmall-4             828 ns/op
BenchmarkGoJayUnsafeDecodeObjSmall-4       745 ns/op

$ cd $GOPATH/src/github.com/francoispqt/gojay/benchmarks/encoder && make bench

BenchmarkGoJayEncodeLargeStruct-4        30490 ns/op
BenchmarkGoJayEncodeMediumStruct-4        1310 ns/op
BenchmarkGoJayEncodeSmallStruct-4          562 ns/op
BenchmarkGoJayEncodeSmallFunc-4            412 ns/op
```
