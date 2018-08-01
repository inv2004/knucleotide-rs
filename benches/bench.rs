
#[macro_use]
extern crate bencher;

extern crate futures;
extern crate futures_cpupool;
extern crate indexmap;
extern crate itertools;
extern crate num;

macro_rules! println {
    () => {()};
    ($fmt:expr) => {()};
    ($fmt:expr, $($arg:tt)*) => {()};
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use bencher::Bencher;
    pub mod main {
        use std;
        include!("../src/bin/main.rs");
    }
    pub mod rust_4 {
        use std;
        include!("../src/bin/rust_4.rs");
    }

    fn bench_knuc_main(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("in250k.txt").unwrap();
            let buf = BufReader::new(file);
            main::calc(buf)
        });
    }

    fn bench_knuc_rust_4(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("in250k.txt").unwrap();
            let buf = BufReader::new(file);
            main::calc(buf)
        });
    }

    benchmark_group!(benches, bench_knuc_main, bench_knuc_rust_4);

}

benchmark_main!(tests::benches);
