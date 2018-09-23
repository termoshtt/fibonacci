extern crate cc;

fn main() {
    cc::Build::new().file("fib.c").compile("fibc");
}
