# dilib-rs

[![Crates.io][crates-badge]][crates-link]
[![License][license-badge]][license-link]
[![Docs][docs-badge]][docs-link]
[![Github-Actions][ci-badge]][ci-link]

[crates-badge]: https://img.shields.io/crates/v/dilib.svg
[crates-link]: https://crates.io/crates/dilib

[license-badge]:  https://img.shields.io/badge/LICENSE-MIT-green.svg
[license-link]: https://github.com/Neo-Ciber94/dilib-rs/blob/master/LICENSE

[docs-badge]: https://img.shields.io/badge/docs-dilib-blue.svg
[docs-link]: https://docs.rs/dilib/latest/dilib/

[ci-badge]: https://github.com/Neo-Ciber94/dilib-rs/actions/workflows/ci.yml/badge.svg
[ci-link]: https://github.com/Neo-Ciber94/dilib-rs/actions

A dependency injection library for Rust.

## Usage
```toml
[dependencies]
dilib = "0.2.0-alpha"
```

## Example

### Basic Usage

```rust
use dilib::Container;

struct Printer;
impl Printer {
    pub fn print(&self, s: &str) {
        println!("{}", s);
    }
}

struct EnglishGreeting;
impl EnglishGreeting {
    pub fn greet(&self) -> String {
        "Hello!".to_string()
    }
}

struct SpanishGreeting;
impl SpanishGreeting {
    pub fn greet(&self) -> String {
        "Hola!".to_string()
    }
}

let mut container = Container::new();
container.add_singleton(Printer).unwrap();
container.add_scoped(|| EnglishGreeting).unwrap();
container.add_scoped_with_name("es", || SpanishGreeting).unwrap();

let printer = container.get::<Printer>().unwrap();
let en = container.get::<EnglishGreeting>().unwrap();
let es = container.get_with_name::<SpanishGreeting>("es").unwrap();

printer.print(&en.greet());
printer.print(&es.greet());
```

## Table of Contents

- [Container](#container)
  - [Scoped provider](#singleton)
  - [Singleton provider](#scoped)
  - [Inject trait](#inject)
  - [Bind trait to implementation](#bind_impl)
  - [get, get_scoped and get_singleton](#get)
- [Derive Inject](#derive_inject)
- [Global Container](#global_container)
- [Provide](#provide)
  - [Why "unstable_provide"?](#why_unstable_provide)
  - [#[provide] macro](#provide_macro)
  - [use of undeclared crate or module `ctor`](#use_of_undeclared_ctor)

## Container
The container is the main storage for the provides,
it stores 2 types of providers:
- `Scoped`: provides a new instance every time it is requested
- `Singleton`: provides a single instance

All this provides can be named or unnamed, using the 
methods that ends with `with_name(...)`.

### Scoped provider
The scoped providers provide a new instance every time they are requested.

```rust
use dilib::Container;

let mut container = Container::new();
container.add_scoped(|| String::from("Apple Pie")).unwrap();

let s = container.get::<String>().unwrap();
assert_eq!(s.as_ref(), "Apple Pie");
```

### Singleton provider
The singleton providers provide a single instance.

```rust
use dilib::Container;
use std::sync::Mutex;

let mut container = Container::new();
container.add_singleton(Mutex::new(0)).unwrap();

{
    let c1 = container.get::<Mutex<i32>>().unwrap();
    *c1.lock().unwrap() = 3;
}

let c2 = container.get::<Mutex<i32>>().unwrap();
assert_eq!(*c2.lock().unwrap(), 3);
```

### Inject trait
The `Inject` trait provide a way to construct a type using the 
providers of the container.

To add a type that implements `Inject` to the container,
you use the `add_deps` methods, this add the type as a `Scoped` provider.

```rust
use std::sync::{Mutex, atomic::AtomicUsize};
use dilib::{Container, Inject};

struct IdGenerator(AtomicUsize);
impl IdGenerator {
  pub fn next(&self) -> usize {
    1 + self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
  }
}

#[derive(Clone, Debug)]
struct Fruit {
    id: usize,
    tag: String
}

impl Inject for Fruit {
    fn inject(container: &Container) -> Self {
      let generator = container.get::<IdGenerator>().unwrap();
      let id = generator.next();
      let tag = container.get_with_name::<String>("fruit").unwrap().cloned();
      Fruit { id, tag }
    }
}

let mut container = Container::new();
container.add_singleton(IdGenerator(AtomicUsize::new(0))).unwrap();
container.add_scoped_with_name("fruit", || String::from("b18ap31")).unwrap();
container.add_deps::<Fruit>().unwrap();

let f1 = container.get::<Fruit>().unwrap();
let f2 = container.get::<Fruit>().unwrap();

assert_eq!(f1.id, 1);
assert_eq!(f1.tag, "b18ap31");

assert_eq!(f2.id, 2);
assert_eq!(f2.tag, "b18ap31");
```

### Bind trait to implementation
Instead of adding a type directly to the container
you can bind a trait to its implementation using the macros:
- `add_scoped_trait!(container, name, trait => impl)`
- `add_singleton_trait!(container, name, trait => impl)`
- `add_scoped_trait!(container, name, trait @ Inject)`
- `add_singleton_trait!(container, name, trait @ Inject)`

The `name` is optional.

And you can get the values back using:
- `get_scoped_trait!(container, name, trait)`
- `get_singleton_trait!(container, name, trait)`
- `get_resolved_trait(container, name, trait)`

The `name` is also optional.

```rust
use dilib::{
  Container,
  add_scoped_trait, 
  add_singleton_trait, 
  get_resolved_trait, 
};

trait Discount {
  fn get_discount(&self) -> f32;
}

trait Fruit {
  fn name(&self) -> &str;
  fn price(&self) -> f32;
}

struct TenPercentDiscount;
impl Discount for TenPercentDiscount {
  fn get_discount(&self) -> f32 {
    0.1
  }
}

struct Apple;
struct Orange;

impl Fruit for Apple {
  fn name(&self) -> &str {
    "Apple"
  }
  
  fn price(&self) -> f32 {
    2.0
  }
}

impl Fruit for Orange {
  fn name(&self) -> &str {
    "Orange"
  }
  
  fn price(&self) -> f32 {
    1.7
  }
}

let mut container = Container::new();
add_singleton_trait!(container, Discount => TenPercentDiscount).unwrap();
add_scoped_trait!(container, "apple", Fruit => Apple).unwrap();
add_scoped_trait!(container, "orange", Fruit => Orange).unwrap();

// All types are returned as `Box<dyn Trait>`
let discount = get_resolved_trait!(container, Discount).unwrap();
let apple = get_resolved_trait!(container, Fruit, "apple").unwrap();
let orange = get_resolved_trait!(container, Fruit, "orange").unwrap();

assert_eq!(discount.get_discount(), 0.1);

assert_eq!(apple.name(), "Apple");
assert_eq!(apple.price(), 2.0);

assert_eq!(orange.name(), "Orange");
assert_eq!(orange.price(), 1.7);
```

### get, get_scoped and get_singleton
There are 3 ways to retrieve a value from the container:
- `get`
- `get_scoped`
- `get_singleton`

And it named variants:
- `get_with_name`
- `get_scoped_with_name`
- `get_singleton_with_name`

`get_scoped` and `get_singleton` are self-explanatory, they get
a value from a `scoped` or `singleton` provider.

But `get` can get any `scoped` and `singleton` value,
the difference is that `get` returns a `Resolved<T>`
and the others returns a `T` or `Arc<T>` for singletons.

`Resolved<T>` is just an enum for a `Scoped(T)` and `Singleton(Arc<T>)`
where you can convert it back using `into_scoped` or `into_singleton`,
the advantage of get is that it implements `Deref` to use the value and its just easier
to call `get`.

## Derive Inject
> This requires the `derive` feature.

Inject is implemented for all types that implement `Default`.
and can be used with `#[derive]`.

```rust
use dilib::{Singleton, Inject, Container};
use dilib_derive::*;

#[derive(Inject)]
struct Apple {
  // Singleton is an alias for Arc<T>
  #[inject(name="apple")]
  tag: Singleton<String>,
  #[inject(name="apple_price")]
  price: f32
}

let mut container = Container::new();
container.add_singleton_with_name("apple", String::from("FRUIT_APPLE")).unwrap();
container.add_scoped_with_name("apple_price", || 2.0_f32).unwrap();
container.add_deps::<Apple>();

let apple = container.get::<Apple>().unwrap();
assert_eq!(apple.tag.as_ref(), "FRUIT_APPLE");
assert_eq!(apple.price, 2.0);
```

## Global Container

> This requires the `global` feature.

`dilib` also offers a global container so you don't require
to declare your own container, and it's easier to access the container
with macros like `get_scoped!`, `get_singleton!`or just `get_resolved!`,
you can also access the container directly using `get_container()`.

```rust
use dilib::{global::init_container, resolve};

init_container(|container| {
    container.add_scoped(|| String::from("Orange")).unwrap();
    container.add_singleton_with_name("num", 123_i32).unwrap();
}).expect("unable to initialize the container");

let orange = resolve!(String).unwrap();
let num = resolve!(i32, "num").unwrap();

assert_eq!(orange.as_ref(), "Orange");
assert_eq!(*num, 123);
```

## Provide
> This requires the `unstable_provide` feature.

### Why "unstable_provide"?
The feature `unstable_provide` make possible to have dependency
injection more similar to other frameworks like C# `EF Core` or Java `Sprint`.

To allow run code before main we use the the [ctor](https://github.com/mmastrac/rust-ctor) crate,
which have been tested in several OS, so depending on where you run your application this feature
may not be unstable for your use case.

### `#[provide]` macro
You can use the `#[provide]` macro over any function or type that implements
**`Inject` to register it to the global container.

<strong>
<small>** Any type that implements `Default` also implements `Inject`.</small>
</strong>

```rust
use std::sync::RwLock;
use dilib::global::init_container;
use dilib::{resolve, Singleton, Inject, provide};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct User {
    name: &'static str,
    email: &'static str,
}

trait Repository<T> {
    fn add(&self, item: T);
    fn get_all(&self) -> Vec<T>;
}

#[derive(Default)]
#[provide(scope="singleton")]
struct Db(RwLock<Vec<User>>);

#[derive(Inject)]
#[provide(bind="Repository<User>")]
struct UserRepository(Singleton<Db>);
impl Repository<User> for UserRepository {
    fn add(&self, item: User) {
        self.0.0.write().unwrap().push(item);
    }

    fn get_all(&self) -> Vec<User> {
        self.0.0.read().unwrap().clone()
    }
}

// Initialize the container to register the providers
init_container(|_container| {
// Add additional providers
}).unwrap();

let user_repository = resolve!(trait Repository<User>).unwrap();
user_repository.add(User { name: "Marie", email: "marie@example.com" });
user_repository.add(User { name: "Natasha", email: "natasha@example.com" });

let users = user_repository.get_all();
let db = resolve!(Db).unwrap();
println!("Total users: {}", db.0.read().unwrap().len());
println!("{:#?}", users);
```