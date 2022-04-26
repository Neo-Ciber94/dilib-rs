use criterion::{criterion_group, Criterion, black_box};
use dilib::Container;

fn get_container() -> Container<'static> {
    let mut container = Container::new();
    container.add_singleton(42_i32).unwrap();
    container.add_singleton_with_name("funny", 69_i32).unwrap();
    container
}

fn singleton_benchmark(c: &mut Criterion) {
    let container = get_container();
    let mut group = c.benchmark_group("Container get singleton");

    group.bench_function("Container::get()", |b| {
        b.iter(|| {
            let s = container.get::<i32>().unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_with_name()", |b| {
        b.iter(|| {
            let s = container.get_with_name::<i32>("funny").unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_singleton()", |b| {
        b.iter(|| {
            let s = container.get_singleton::<i32>().unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_singleton_with_name()", |b| {
        b.iter(|| {
            let s = container.get_singleton_with_name::<i32>("funny").unwrap();
            black_box(s);
        })
    });

    group.finish();
}

criterion_group!(singleton_benches, singleton_benchmark);