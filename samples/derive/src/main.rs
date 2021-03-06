#![allow(dead_code)]
use colored::Colorize;
use dilib::Container;
use fruits::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Color {
    Red,
    Yellow,
    Orange,
    Green,
}

#[derive(Debug, Clone)]
pub struct Fruit {
    name: &'static str,
    color: Color,
}

fn main() {
    let mut container = Container::new();

    // Register the `Db` as a singleton
    container.add_singleton(Db::new(vec![])).unwrap();

    // Register the repository, service and initializer as scoped values
    container.add_deps::<FruitRepository>().unwrap();
    container.add_deps::<FruitService>().unwrap();
    container.add_deps::<DbInitializer>().unwrap();

    // Gets the initializer, this could just be a singleton
    let initializer = container.get::<DbInitializer>().unwrap();
    initializer
        .init()
        .expect("Failed to initialize the database");

    // Gets the service, calling `get` will returns a new instance of the service each time
    let fruit_service = container.get::<FruitService>().unwrap();
    fruit_service.add_all(vec![
        Fruit {
            name: "Grapes",
            color: Color::Green,
        },
        Fruit {
            name: "Raspberries",
            color: Color::Red,
        },
    ]);

    let red_fruits = fruit_service.get_all_by_color(Some(Color::Red));
    println!(
        "Red Fruits: {}",
        format!("{:#?}", red_fruits).as_str().bright_red()
    );

    let yellow_fruits = fruit_service.get_all_by_color(Some(Color::Yellow));
    println!(
        "Yellow Fruits: {}",
        format!("{:#?}", yellow_fruits).as_str().bright_yellow()
    );

    let all_fruits = fruit_service.get_all_by_color(None);
    println!(
        "All Fruits: {}",
        format!("{:#?}", all_fruits).as_str().bright_cyan()
    );
}

mod fruits {
    use crate::{Color, Fruit};
    use dilib::{Inject, Singleton};
    use std::sync::RwLock;

    // The in memory database used to store the fruits
    pub type Db = RwLock<Vec<Fruit>>;

    pub trait Repository<T> {
        fn get_all(&self) -> Vec<T>;
        fn add(&self, item: T);
    }

    #[derive(Inject)]
    pub struct FruitRepository {
        // Injects the `Db` singleton, `Singleton<T>` is just an alias for `Arc<T>`,
        // if we just use `Db` type it will try to inject it as a scoped value, so will fail with an error
        // because the `Db` is registered as a singleton.
        db: Singleton<Db>,
    }

    impl Repository<Fruit> for FruitRepository {
        fn get_all(&self) -> Vec<Fruit> {
            self.db.read().unwrap().clone()
        }

        fn add(&self, fruit: Fruit) {
            self.db.write().unwrap().push(fruit);
        }
    }

    #[derive(Inject)]
    pub struct FruitService {
        // Injects a new `FruitRepository` scoped instance
        repository: FruitRepository,
    }

    impl FruitService {
        pub fn add_all<I>(&self, fruits: I)
        where
            I: IntoIterator<Item = Fruit>,
        {
            for fruit in fruits {
                self.repository.add(fruit);
            }
        }

        pub fn get_all_by_color(&self, color: Option<Color>) -> Vec<Fruit> {
            let fruits = self.repository.get_all();
            match color {
                Some(color) => fruits.into_iter().filter(|f| f.color == color).collect(),
                None => fruits,
            }
        }
    }

    // Injects the `Db` singleton
    #[derive(Inject)]
    pub struct DbInitializer(Singleton<Db>);

    #[derive(Debug)]
    pub struct AlreadyInitialized;

    impl DbInitializer {
        pub fn init(&self) -> Result<(), AlreadyInitialized> {
            let db = &self.0;

            if db.read().unwrap().len() > 0 {
                return Err(AlreadyInitialized);
            }

            let fruits = vec![
                Fruit {
                    name: "Apple",
                    color: Color::Red,
                },
                Fruit {
                    name: "Banana",
                    color: Color::Yellow,
                },
                Fruit {
                    name: "Orange",
                    color: Color::Orange,
                },
                Fruit {
                    name: "Pear",
                    color: Color::Green,
                },
                Fruit {
                    name: "Strawberry",
                    color: Color::Red,
                },
                Fruit {
                    name: "Watermelon",
                    color: Color::Green,
                },
                Fruit {
                    name: "Kiwi",
                    color: Color::Green,
                },
                Fruit {
                    name: "Pineapple",
                    color: Color::Yellow,
                },
                Fruit {
                    name: "Mango",
                    color: Color::Orange,
                },
                Fruit {
                    name: "Cherry",
                    color: Color::Red,
                },
                Fruit {
                    name: "Papaya",
                    color: Color::Yellow,
                },
                Fruit {
                    name: "Avocado",
                    color: Color::Green,
                },
                Fruit {
                    name: "Pomegranate",
                    color: Color::Red,
                },
                Fruit {
                    name: "Passionfruit",
                    color: Color::Orange,
                },
                Fruit {
                    name: "Coconut",
                    color: Color::Green,
                },
                Fruit {
                    name: "Lemon",
                    color: Color::Yellow,
                },
                Fruit {
                    name: "Lime",
                    color: Color::Green,
                },
            ];

            db.write().unwrap().extend(fruits);
            Ok(())
        }
    }
}
