#[macro_use]
extern crate timeit;

mod j1;

fn main() {
    timeit!({j1::p1();});
}