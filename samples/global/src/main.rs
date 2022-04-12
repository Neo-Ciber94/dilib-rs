use std::sync::{Arc, RwLock};
use dilib::{Inject, Singleton, provide, resolve};
use dilib::global::init_container;

#[derive(Clone, Default)]
#[provide(scope="singleton")]
struct Db(Arc<RwLock<Vec<String>>>);

#[derive(Inject)]
#[provide]
struct Repository(Singleton<Db>);

impl Repository {
    pub fn get_all(&self) -> Vec<String> {
        let db = &self.0;
        db.0.read().unwrap().clone()
    }

    pub fn set(&self, data: String) {
        let db = &*self.0;
        db.0.write().unwrap().push(data);
    }
}

fn main() {
    init_container(|_|{}).unwrap();

    let repo1 = resolve!(Repository).unwrap();
    repo1.set("2".to_string());
    repo1.set("4".to_string());
    repo1.set("6".to_string());

    let repo2 = resolve!(Repository).unwrap();
    let items = repo2.get_all();
    println!("{:?}", items);
}

