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

fn main() {
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
}
