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
mod j10;
mod j11;
mod j12;

fn bench(c: &mut Criterion) {

    let mut group = c.benchmark_group("All");
    group.bench_function("j01p1", |b| b.iter(|| { j1::_p1(black_box(include_str!("j1/j1.txt"))); }));
    group.bench_function("j01p2", |b| b.iter(|| { j1::_p2(black_box(include_str!("j1/j1.txt"))); }));
    group.bench_function("j02p1", |b| b.iter(|| { j2::_p1(black_box(include_str!("j2/j2.txt"))); }));
    group.bench_function("j02p2", |b| b.iter(|| { j2::_p2(black_box(include_str!("j2/j2.txt"))); }));
    group.bench_function("j03p1", |b| b.iter(|| { j3::_p1(black_box(include_str!("j3/j3.txt"))); }));
    group.bench_function("j03p2", |b| b.iter(|| { j3::_p2(black_box(include_str!("j3/j3.txt"))); }));
    group.bench_function("j04p1", |b| b.iter(|| { j4::_p1(black_box(include_str!("j4/j4.txt"))); }));
    group.bench_function("j04p2", |b| b.iter(|| { j4::_p2(black_box(include_str!("j4/j4.txt"))); }));
    group.bench_function("j05p1", |b| b.iter(|| { j5::_p1(black_box(include_str!("j5/j5.txt"))); }));
    group.bench_function("j05p2", |b| b.iter(|| { j5::_p2(black_box(include_str!("j5/j5.txt"))); }));
    group.bench_function("j06p1", |b| b.iter(|| { j6::_p1(black_box(include_str!("j6/j6.txt"))); }));
    group.bench_function("j06p2", |b| b.iter(|| { j6::_p2(black_box(include_str!("j6/j6.txt"))); }));
    group.bench_function("j07p1", |b| b.iter(|| { j7::_p1(black_box(include_str!("j7/j7.txt"))); }));
    group.bench_function("j07p2", |b| b.iter(|| { j7::_p2(black_box(include_str!("j7/j7.txt"))); }));
    group.bench_function("j08p1", |b| b.iter(|| { j8::_p1(black_box(include_str!("j8/j8.txt"))); }));
    group.bench_function("j08p2", |b| b.iter(|| { j8::_p2(black_box(include_str!("j8/j8.txt"))); }));
    group.bench_function("j09p1", |b| b.iter(|| { j9::_p1(black_box(include_str!("j9/j9.txt"))); }));
    group.bench_function("j09p2", |b| b.iter(|| { j9::_p2(black_box(include_str!("j9/j9.txt"))); }));
    group.bench_function("j10p1", |b| b.iter(|| { j10::_p1(black_box(include_str!("j10/j10.txt"))); }));
    group.bench_function("j10p2", |b| b.iter(|| { j10::_p2(black_box(include_str!("j10/j10.txt"))); }));
    group.bench_function("j11p1", |b| b.iter(|| { j11::_p1(black_box(include_str!("j11/j11.txt"))); }));
    group.bench_function("j11p2", |b| b.iter(|| { j11::_p1(black_box(include_str!("j11/j11.txt"))); }));
    group.bench_function("j12p1", |b| b.iter(|| { j12::_p1(black_box(include_str!("j12/j12.txt"))); }));
    group.bench_function("j12p2", |b| b.iter(|| { j12::_p2(black_box(include_str!("j12/j12.txt"))); }));
}

criterion_group!(benches, bench);
criterion_main!(benches);