
extern crate bencher;

use bencher::Bencher;

fn pow(a: usize, b: usize) -> usize {
    let mut result = 1;
    for _i in 0..b {
        result *= a;
    }
    result
}

fn basic_bench(b: &mut Bencher) {
    let mut runtime = aklete::Aklete::new();
    b.iter(|| {
        for _i in 0..1_000 {
            runtime.spawn(async {
                pow(2, 20);
                Ok(())
            });
        }
        runtime.run().unwrap();
    }); 
}

bencher::benchmark_group!(basic, basic_bench);

bencher::benchmark_main!(basic);
