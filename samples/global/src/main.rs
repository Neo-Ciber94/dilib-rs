use std::sync::RwLock;
use dilib::{resolve, provide, Singleton, Inject, global::init_container};

#[derive(Debug, Clone, Eq, PartialEq)]
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
    fn add(&self, user: User) {
        self.0.0.write().unwrap().push(user);
    }

    fn get_all(&self) -> Vec<User> {
        self.0.0.read().unwrap().clone()
    }
}

fn main() {
    init_container(|_container| {
        // Adds other dependencies to the container here
    }).unwrap();

    let repository = resolve!(trait Repository<User>).expect("unable to get UserRepository");
    repository.add(User { name: "Marie", email: "marie@example.com" });
    repository.add(User { name: "Natasha", email: "natasha@example.com" });

    let users = repository.get_all();
    println!("{:#?}", users);
}