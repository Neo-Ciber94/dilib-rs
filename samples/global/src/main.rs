use std::sync::{Arc, RwLock};
use dilib::{Inject, Singleton, provide, resolve};
use dilib::global::init_container;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Data(u64);

#[derive(Clone, Default)]
#[provide(scope="singleton")]
struct Db(Arc<RwLock<Vec<Data>>>);

#[derive(Inject)]
#[provide]
struct Repository(Singleton<Db>);

impl Repository {
    pub fn get_all(&self) -> Vec<Data> {
        let db = &self.0;
        db.0.read().unwrap().clone()
    }

    // pub fn get(&self, id: u64) -> Option<Data> {
    //     let db = &*self.0;
    //     db.0.read().unwrap().iter().find(|d| d.0 == id).map(|d| d.clone())
    // }

    pub fn set(&self, id: u64) {
        let db = &*self.0;
        db.0.write().unwrap().push(Data(id));
    }
}

fn main() {
    init_container(|_|{}).unwrap();

    let repo1 = resolve!(Repository).unwrap();
    repo1.set(2);
    repo1.set(4);
    repo1.set(6);

    let repo2 = resolve!(Repository).unwrap();
    let items = repo2.get_all();
    println!("{:?}", items);
}

