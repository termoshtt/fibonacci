#![feature(test)]

extern crate fib;
extern crate test;

use test::Bencher;

#[bench]
fn fib_50(b: &mut Bencher) {
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
fn fib_cpp_50(b: &mut Bencher) {
    b.iter(|| unsafe {
        let _val = fib::fib_cpp(50);
    })
}
