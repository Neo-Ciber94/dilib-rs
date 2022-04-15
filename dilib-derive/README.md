# Dilib derive

Provides the `#[derive(Inject)]` attribute to implement the `Inject` trait.

```rust
use dilib::{Inject, Singleton};

struct User(String);

#[derive(Inject)]
struct UserService {
    db: Singleton<Vec<User>>,
}
```