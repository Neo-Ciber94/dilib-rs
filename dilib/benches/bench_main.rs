use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::scoped::scoped_benches,
    benchmarks::singleton::singleton_benches,
    benchmarks::singleton_lazy::singleton_lazy_benches,
    benchmarks::get_named_comparison::get_named_benches
}
