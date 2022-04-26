use criterion::{criterion_group,Criterion, black_box};
use dilib::Container;

fn get_container() -> Container<'static> {
    let mut container = Container::new();
    container.add_scoped(|| String::from("hello")).unwrap();
    container.add_scoped_with_name("greet", || String::from("Salutations")).unwrap();
    container
}

fn scoped_benchmark(c: &mut Criterion) {
    let container = get_container();
    let mut group = c.benchmark_group("Container get scoped");

    group.bench_function("Container::get()", |b| {
        b.iter(|| {
            let s = container.get::<String>().unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_with_name()", |b| {
        b.iter(|| {
            let s = container.get_with_name::<String>("greet").unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_scoped()", |b| {
        b.iter(|| {
            let s = container.get_scoped::<String>().unwrap();
            black_box(s);
        })
    });

    group.bench_function("Container::get_scoped_with_name()", |b| {
        b.iter(|| {
            let s = container.get_scoped_with_name::<String>("greet").unwrap();
            black_box(s);
        })
    });

    group.finish();
}

criterion_group!(scoped_benches, scoped_benchmark);