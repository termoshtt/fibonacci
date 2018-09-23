extern crate cc;

fn main() {
    cc::Build::new().file("fib.cpp").compile("fibc");
}
