# Dilib derive

Provides the `#[derive(Inject)]` attribute to implement the `Inject` trait.


## Usage
```rust
use dilib::{Inject, Singleton};

struct User(String);

#[derive(Inject)]
struct UserService {
    db: Singleton<Vec<User>>,
}
```

## License
This project is licensed under the [MIT license](https://github.com/Neo-Ciber94/dilib-rs/blob/master/LICENSE).