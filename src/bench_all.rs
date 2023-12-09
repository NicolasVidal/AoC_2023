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

    let mut group = c.benchmark_group("All");
    group.bench_function("j1p1", |b| b.iter(|| { j1::_p1(black_box(include_str!("j1/j1.txt"))); }));
    group.bench_function("j1p2", |b| b.iter(|| { j1::_p2(black_box(include_str!("j1/j1.txt"))); }));
    group.bench_function("j2p1", |b| b.iter(|| { j2::_p1(black_box(include_str!("j2/j2.txt"))); }));
    group.bench_function("j2p2", |b| b.iter(|| { j2::_p2(black_box(include_str!("j2/j2.txt"))); }));
    group.bench_function("j3p1", |b| b.iter(|| { j3::_p1(black_box(include_str!("j3/j3.txt"))); }));
    group.bench_function("j3p2", |b| b.iter(|| { j3::_p2(black_box(include_str!("j3/j3.txt"))); }));
    group.bench_function("j4p1", |b| b.iter(|| { j4::_p1(black_box(include_str!("j4/j4.txt"))); }));
    group.bench_function("j4p2", |b| b.iter(|| { j4::_p2(black_box(include_str!("j4/j4.txt"))); }));
    group.bench_function("j5p1", |b| b.iter(|| { j5::_p1(black_box(include_str!("j5/j5.txt"))); }));
    group.bench_function("j5p2", |b| b.iter(|| { j5::_p2(black_box(include_str!("j5/j5.txt"))); }));
    group.bench_function("j6p1", |b| b.iter(|| { j6::_p1(black_box(include_str!("j6/j6.txt"))); }));
    group.bench_function("j6p2", |b| b.iter(|| { j6::_p2(black_box(include_str!("j6/j6.txt"))); }));
    group.bench_function("j7p1", |b| b.iter(|| { j7::_p1(black_box(include_str!("j7/j7.txt"))); }));
    group.bench_function("j7p2", |b| b.iter(|| { j7::_p2(black_box(include_str!("j7/j7.txt"))); }));
    group.bench_function("j8p1", |b| b.iter(|| { j8::_p1(black_box(include_str!("j8/j8.txt"))); }));
    group.bench_function("j8p2", |b| b.iter(|| { j8::_p2(black_box(include_str!("j8/j8.txt"))); }));
    group.bench_function("j9p1", |b| b.iter(|| { j9::_p1(black_box(include_str!("j9/j9.txt"))); }));
    group.bench_function("j9p2", |b| b.iter(|| { j9::_p2(black_box(include_str!("j9/j9.txt"))); }));
}

criterion_group!(benches, bench);
criterion_main!(benches);