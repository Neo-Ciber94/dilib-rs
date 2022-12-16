use dilib::{Container, Inject};

#[derive(Default, Debug)]
struct Engine;

#[derive(Default, Debug, Clone, Copy)]
struct Wheel;

#[allow(dead_code)]
#[derive(Debug, Inject)]
struct Car {
    engine: Engine,
    wheel: [Wheel; 4],
}

fn main() {
    let mut container = Container::new();
    container.add_scoped(Engine::default).unwrap();
    container.add_scoped(|| [Wheel; 4]).unwrap();
    container.add_deps::<Car>().unwrap();

    let car = container.get::<Car>().unwrap();
    println!("{:#?}", car);
}
