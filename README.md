# knucleotide-rs
benchmark for benchmarksgame-team.pages.debian.net

## run
cargo run --bin fasta --release -- 25000000 > in25m.txt

cargo run --bin main --release -- < in25m.txt
