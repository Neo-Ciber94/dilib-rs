use dilib::{Singleton, Injectable};
struct DefaultTyParam<A = usize, B = String, C = bool> {
    a: A,
    b: Singleton<B>,
    c: Box<C>,
}
