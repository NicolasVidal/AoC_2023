#[macro_use]
extern crate timeit;

mod j1;
mod j2;
mod j3;
mod j4;
mod j5;
mod j6;
mod j7;
mod j8;
mod j9;

fn main() {
    timeit!({j1::p1();});
    timeit!({j1::p2();});
    timeit!({j2::p1();});
    timeit!({j2::p2();});
    timeit!({j3::p1();});
    timeit!({j3::p2();});
    timeit!({j4::p1();});
    timeit!({j4::p2();});
    timeit!({j5::p1();});
    timeit!({j5::p2();});
    timeit!({j6::p1();});
    timeit!({j6::p2();});
    timeit!({j7::p1();});
    timeit!({j7::p2();});
    timeit!({j8::p1();});
    timeit!({j8::p2();});
    timeit!({j9::p1();});
    timeit!({j9::p2();});
}