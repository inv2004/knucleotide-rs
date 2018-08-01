
#[macro_use]
extern crate bencher;

extern crate futures;
extern crate futures_cpupool;
extern crate indexmap;
extern crate itertools;
extern crate num;

macro_rules! println {
    () => {};
    ($fmt:expr) => {};
    ($fmt:expr, $($arg:tt)*) => {};
}

#[cfg(test)]
mod tests {
    use std;
    include!("../src/bin/main.rs_");
    use bencher::Bencher;

    fn bench_knuc_main(b: &mut Bencher) {
        b.iter(|| {

            let file = File::open("in250k.txt").unwrap();
            let buf = BufReader::new(file);
            calc(buf)
        });
    }

    benchmark_group!(benches, bench_knuc_main);
}

benchmark_main!(tests::benches);
