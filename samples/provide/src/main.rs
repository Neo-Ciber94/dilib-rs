use crate::app::DbSeeder;
use app::Repository;
use dilib::{global::init_container, resolve};
use std::collections::HashMap;
use std::num::NonZeroUsize;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Food {
    name: &'static str,
    price: f64,
    restaurant: NonZeroUsize,
}

#[derive(Debug, Clone)]
pub struct Restaurant {
    id: NonZeroUsize,
    name: &'static str,
}

fn main() {
    // Initialize the container to register the providers
    init_container(|_container| {
        // Add additional providers
    })
    .unwrap();

    // Gets the database initializer
    let seeder = resolve!(DbSeeder).expect("Failed to resolve DbSeeder");
    seeder.init_db().unwrap();

    let food_repository =
        resolve!(trait Repository<Food>).expect("Failed to resolve FoodRepository");

    let restaurant_repository =
        resolve!(trait Repository<Restaurant>).expect("Failed to resolve RestaurantRepository");

    // Makes a join to get the restaurant menus
    let restaurant_menus = restaurant_repository
        .get_all()
        .iter()
        .map(|res| {
            let foods = food_repository
                .get_all()
                .iter()
                .filter(|f| f.restaurant == res.id)
                .cloned()
                .collect::<Vec<Food>>();

            (res.name, foods)
        })
        .collect::<HashMap<&'static str, Vec<Food>>>();

    println!("{:#?}", restaurant_menus);
}

mod app {
    use crate::{Food, Restaurant};
    use dilib::{provide, Inject, Singleton};
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::num::NonZeroUsize;
    use std::sync::RwLock;

    pub trait Repository<T> {
        fn add(&self, item: T);
        fn get_all(&self) -> Vec<T>;
    }

    // We just use a HashMap as a database
    #[derive(Default)]
    #[provide(scope = "singleton")]
    pub struct Db(RwLock<HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>>);
    impl Db {
        pub fn add_all<I, T>(&self, new_items: I)
        where
            I: IntoIterator<Item = T>,
            T: Send + Sync + Clone + 'static,
        {
            let mut lock = self.0.write().unwrap();
            let items = lock.entry(TypeId::of::<T>()).or_insert_with(Vec::new);

            for item in new_items {
                items.push(Box::new(item));
            }
        }

        pub fn get_all<T>(&self) -> Vec<T>
        where
            T: Any + Send + Sync + Clone + 'static,
        {
            let mut items = Vec::new();
            let lock = self.0.read().unwrap();
            if let Some(any_vec) = lock.get(&TypeId::of::<T>()) {
                let vec = any_vec
                    .iter()
                    .map(|any| any.downcast_ref::<T>().cloned())
                    .flatten()
                    .collect::<Vec<T>>();

                items.extend(vec);
            }
            items
        }
    }

    // Provides an implementation for `Repository<Food>`
    #[derive(Inject)]
    #[provide(bind = "Repository<Food>")]
    pub struct FoodRepository(Singleton<Db>);
    impl Repository<Food> for FoodRepository {
        fn add(&self, item: Food) {
            self.0.add_all([item]);
        }

        fn get_all(&self) -> Vec<Food> {
            self.0.get_all()
        }
    }

    // Provides an implementation for `Repository<Restaurant>`
    #[derive(Inject)]
    #[provide(bind = "Repository<Restaurant>")]
    pub struct RestaurantRepository(Singleton<Db>);
    impl Repository<Restaurant> for RestaurantRepository {
        fn add(&self, item: Restaurant) {
            self.0.add_all([item]);
        }

        fn get_all(&self) -> Vec<Restaurant> {
            self.0.get_all()
        }
    }

    #[derive(Debug)]
    pub struct AlreadyInitialized;

    // This could just be a function, but is a singleton for the sake of the example
    #[derive(Inject)]
    #[provide(scope = "singleton")]
    pub struct DbSeeder(Singleton<Db>);
    impl DbSeeder {
        pub fn init_db(&self) -> Result<(), AlreadyInitialized> {
            if self.0 .0.read().unwrap().len() > 0 {
                return Err(AlreadyInitialized);
            }

            let restaurants = vec![
                Restaurant {
                    id: NonZeroUsize::new(1).unwrap(),
                    name: "Pizza Hut",
                },
                Restaurant {
                    id: NonZeroUsize::new(2).unwrap(),
                    name: "McDonalds",
                },
            ];

            let foods = vec![
                Food {
                    name: "Pizza",
                    price: 20.0,
                    restaurant: restaurants[0].id,
                },
                Food {
                    name: "Pasta",
                    price: 15.0,
                    restaurant: restaurants[0].id,
                },
                Food {
                    name: "Fries",
                    price: 2.0,
                    restaurant: restaurants[1].id,
                },
                Food {
                    name: "Burger",
                    price: 5.0,
                    restaurant: restaurants[1].id,
                },
            ];

            self.0.add_all(foods);
            self.0.add_all(restaurants);

            Ok(())
        }
    }
}
