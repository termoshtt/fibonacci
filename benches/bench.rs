#![feature(test)]

extern crate fib;
extern crate test;

use test::Bencher;

#[bench]
fn fib50(b: &mut Bencher) {
    b.iter(|| {
        let _val = fib::fib_memo(50);
    })
}

#[bench]
fn fib_static_50(b: &mut Bencher) {
    b.iter(|| unsafe {
        let _val = fib::fib_static(50);
    })
}

#[bench]
fn fib_c50(b: &mut Bencher) {
    b.iter(|| unsafe {
        let _val = fib::fib_c(50);
    })
}
