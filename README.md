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

BenchmarkGoJayDecodeObjLarge-4          	   30000	     52713 ns/op	   30982 B/op	      77 allocs/op
BenchmarkGoJayUnsafeDecodeObjLarge-4    	   30000	     46322 ns/op	    2560 B/op	      76 allocs/op
BenchmarkGoJayDecodeObjMedium-4         	  300000	      4708 ns/op	    2448 B/op	       8 allocs/op
BenchmarkGoJayUnsafeDecodeObjMedium-4   	  300000	      4096 ns/op	     144 B/op	       7 allocs/op
BenchmarkGoJayDecodeObjSmall-4          	 2000000	       828 ns/op	     256 B/op	       2 allocs/op
BenchmarkGoJayUnsafeDecodeObjSmall-4    	 2000000	       745 ns/op	     112 B/op	       1 allocs/op

$ cd $GOPATH/src/github.com/francoispqt/gojay/benchmarks/encoder && make bench

BenchmarkGoJayEncodeLargeStruct-4           	   50000	     30490 ns/op	    9830 B/op	     418 allocs/op
BenchmarkGoJayEncodeMediumStruct-4          	 1000000	      1310 ns/op	     312 B/op	      14 allocs/op
BenchmarkGoJayEncodeSmallStruct-4           	 3000000	       562 ns/op	     112 B/op	       1 allocs/op
BenchmarkGoJayEncodeSmallFunc-4             	 3000000	       412 ns/op	       0 B/op	       0 allocs/op
```
