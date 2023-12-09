use criterion::{black_box, Criterion, criterion_group, criterion_main};

mod j1;
mod j2;
mod j3;
mod j4;
mod j5;
mod j6;
mod j7;
mod j8;
mod j9;


fn bench(c: &mut Criterion) {
    c.bench_function("all", |b| b.iter(|| {
        j1::_p1(black_box(include_str!("j1/j1.txt")));
        j1::_p2(black_box(include_str!("j1/j1.txt")));
        j2::_p1(black_box(include_str!("j2/j2.txt")));
        j2::_p2(black_box(include_str!("j2/j2.txt")));
        j3::_p1(black_box(include_str!("j3/j3.txt")));
        j3::_p2(black_box(include_str!("j3/j3.txt")));
        j4::_p1(black_box(include_str!("j4/j4.txt")));
        j4::_p2(black_box(include_str!("j4/j4.txt")));
        j5::_p1(black_box(include_str!("j5/j5.txt")));
        j5::_p2(black_box(include_str!("j5/j5.txt")));
        j6::_p1(black_box(include_str!("j6/j6.txt")));
        j6::_p2(black_box(include_str!("j6/j6.txt")));
        j7::_p1(black_box(include_str!("j7/j7.txt")));
        j7::_p2(black_box(include_str!("j7/j7.txt")));
        j8::_p1(black_box(include_str!("j8/j8.txt")));
        j8::_p2(black_box(include_str!("j8/j8.txt")));
        j9::_p1(black_box(include_str!("j9/j9.txt")));
        j9::_p2(black_box(include_str!("j9/j9.txt")));
    }));
}

criterion_group!(benches, bench);
criterion_main!(benches);