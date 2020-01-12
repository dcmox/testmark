extern crate testmark;
use testmark::Testmark;
use testmark::Timer;

fn main() {
    let mut bench: Testmark = Timer::new("Loop test");
    bench.start();
    let mut x: u64 = 42;
    for i in 0..10000000 {
        x += i;
    }
    bench.stop();
    bench.print();
}