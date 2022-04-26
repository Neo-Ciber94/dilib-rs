use criterion::{criterion_group, Criterion, black_box};
use dilib::Container;

fn get_container() -> Container<'static> {
    let mut container = Container::new();
    container.add_scoped_with_name("num1", || 123_i32).unwrap();
    container.add_singleton_with_name("num2", 456_i32).unwrap();
    container.add_lazy_singleton_with_name("num3", |_| 789_i32).unwrap();
    container
}

fn get_named_comparisons(c: &mut Criterion) {
    let container = get_container();
    let mut group = c.benchmark_group("Container get named comparisons");

    group.bench_function("Container::get_scoped_with_name()", |b| {
        b.iter(|| {
            let s = container.get_scoped_with_name::<i32>("num1").unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_singleton_with_name()", |b| {
        b.iter(|| {
            let s = container.get_singleton_with_name::<i32>("num2").unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_lazy_singleton_with_name()", |b| {
        b.iter(|| {
            let s = container.get_singleton_with_name::<i32>("num3").unwrap();
            black_box(s);
        })
    });

    group.finish();
}

criterion_group!(get_named_benches, get_named_comparisons);