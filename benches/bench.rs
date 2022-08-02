use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smallvec::SmallVec;

#[derive(Clone, Copy)]
struct S {
    a: usize,
    b: f64,
}

fn usize_8(count: usize) -> SmallVec<[usize; 8]> {
    SmallVec::from_iter(std::iter::repeat(1).take(count))
}

fn usize_32(count: usize) -> SmallVec<[usize; 32]> {
    SmallVec::from_iter(std::iter::repeat(1).take(count))
}

fn usize_64(count: usize) -> SmallVec<[usize; 64]> {
    SmallVec::from_iter(std::iter::repeat(1).take(count))
}

fn two_usize_8(count: usize) -> SmallVec<[(usize, usize); 8]> {
    SmallVec::from_iter(std::iter::repeat((1, 2)).take(count))
}

fn two_usize_32(count: usize) -> SmallVec<[(usize, usize); 32]> {
    SmallVec::from_iter(std::iter::repeat((1, 2)).take(count))
}

fn two_usize_64(count: usize) -> SmallVec<[(usize, usize); 32]> {
    SmallVec::from_iter(std::iter::repeat((1, 2)).take(count))
}

fn s_8(count: usize) -> SmallVec<[S; 8]> {
    SmallVec::from_iter(std::iter::repeat(S { a: 1, b: 2.3 }).take(count))
}

fn s_32(count: usize) -> SmallVec<[S; 32]> {
    SmallVec::from_iter(std::iter::repeat(S { a: 1, b: 2.3 }).take(count))
}

fn s_64(count: usize) -> SmallVec<[S; 64]> {
    SmallVec::from_iter(std::iter::repeat(S { a: 1, b: 2.3 }).take(count))
}

macro_rules! bench_functions {
    ($group: ident, $f: ident, $count: expr) => {
        $group.bench_function("value", |b| {
            b.iter(|| {
                let values = black_box($f($count));
                for value in values {
                    black_box(value);
                }
            })
        });
        $group.bench_function("ref", |b| {
            b.iter(|| {
                let values = black_box($f($count));
                for value in &values {
                    black_box(value);
                }
            })
        });
        $group.bench_function("ref_deref", |b| {
            b.iter(|| {
                let values = black_box($f($count));
                for &value in &values {
                    black_box(value);
                }
            })
        });
    };
}

macro_rules! bench {
    ($bench_f: ident, $f: ident, $($count:expr),+) => {
        fn $bench_f(criterion: &mut Criterion) {
            let args = [$(($f, $count),)+];
            for (f, c) in args {
                let mut group = criterion.benchmark_group(format!("{}_{c}", stringify!($f)));
                bench_functions!(group, f, c);
            }
        }
    };
}

bench!(bench_usize_8, usize_8, 2, 8);
bench!(bench_usize_32, usize_32, 2, 8, 32);
bench!(bench_usize_64, usize_64, 2, 8, 32, 64);
bench!(bench_two_usize_8, two_usize_8, 2, 8);
bench!(bench_two_usize_32, two_usize_32, 2, 8, 32);
bench!(bench_two_usize_64, two_usize_64, 2, 8, 32, 64);
bench!(bench_s_8, s_8, 2, 8);
bench!(bench_s_32, s_32, 2, 8, 32);
bench!(bench_s_64, s_64, 2, 8, 32, 64);

criterion_group!(
    benches,
    bench_usize_8,
    bench_usize_32,
    bench_usize_64,
    bench_two_usize_8,
    bench_two_usize_32,
    bench_two_usize_64,
    bench_s_8,
    bench_s_32,
    bench_s_64,
);
criterion_main!(benches);
