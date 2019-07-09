# knucleotide-rs
benchmark for https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/knucleotide.html

## run

### for bench
cargo run --bin fasta --release -- 250000 > in250k.txt

cargo bench

### results for in250k.txt
```
test bench_knuc_main   ... bench:   2,803,175 ns/iter (+/- 412,599)
test bench_knuc_rust_4 ... bench:   6,995,310 ns/iter (+/- 845,956)
```
