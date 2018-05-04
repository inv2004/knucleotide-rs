# knucleotide-rs
benchmark for https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/knucleotide.html

## run

### for bench
cargo run --bin fasta --release -- 250000 > in250k.txt

cargo bench --bin main --

### for run
cargo run --bin fasta --release -- 25000000 > in25m.txt

cargo run --bin main --release -- < in25m.txt

### results for in250k.txt
```
test tests::bench_knuc_main ... bench:  33,017,369 ns/iter (+/- 820,052)
test tests::bench_knuc_rust_6 ... bench:  35,605,341 ns/iter (+/- 1,115,221)
```
