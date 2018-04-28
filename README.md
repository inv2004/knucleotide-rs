# knucleotide-rs
benchmark for https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/knucleotide.html

## run

### for bench
cargo run --bin fasta --release -- 250000 > in250k.txt

cargo bench --bin main --

### for run
cargo run --bin fasta --release -- 25000000 > in25m.txt

cargo run --bin main --release -- < in25m.txt
