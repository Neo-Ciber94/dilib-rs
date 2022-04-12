// use dilib::global::init_container;
// use dilib::{provide, resolve, Inject, Singleton};
// use std::sync::{Arc, RwLock};
//
// #[derive(Clone, Default)]
// #[provide(scope = "singleton")]
// struct Db(Arc<RwLock<Vec<String>>>);
//
// #[derive(Inject)]
// #[provide]
// struct Repository(Singleton<Db>);
//
// const _ : () = {
//     #[ctor::ctor]
//     fn _test_fn() {
//         println!("Before main 1");
//     }
// };


fn dilib__test_fn_string_8c959fc1616460c7c93522fa189b785e6fccd0c8() -> String {
    "Hello".to_string()
}


// impl Repository {
//     pub fn get_all(&self) -> Vec<String> {
//         let db = &self.0;
//         db.0.read().unwrap().clone()
//     }
//
//     pub fn set(&self, data: String) {
//         let db = &*self.0;
//         db.0.write().unwrap().push(data);
//     }
// }

fn main() {
    println!("Hello, world!");
    // init_container(|_| {}).unwrap();
    //
    // let repo1 = resolve!(Repository).unwrap();
    // repo1.set("2".to_string());
    // repo1.set("4".to_string());
    // repo1.set("6".to_string());
    //
    // let repo2 = resolve!(Repository).unwrap();
    // let items = repo2.get_all();
    // println!("{:?}", items);
}
