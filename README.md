# knucleotide-rs
benchmark for https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/knucleotide.html

## run

### for bench
cargo run --bin fasta --release -- 250000 > in250k.txt

cargo bench

### results for in250k.txt
```
test bench_knuc_main   ... bench:   6,553,490 ns/iter (+/- 335,333)
test bench_knuc_rust_4 ... bench:   6,861,270 ns/iter (+/- 357,601)
```
