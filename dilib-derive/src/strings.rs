
// #[inject]
pub const INJECT : &str = "inject";

// #[inject(constructor="name(param1, param2, ...)")]
pub const CONSTRUCTOR : &str = "constructor";

// #[inject(scope="singleton")]
pub const SCOPE : &str = "scope";

// #[inject(name="API_KEY")]
pub const NAME : &str = "name";

// #[inject(default=1)]
pub const DEFAULT : &str = "default";

// #[inject(scope="singleton")]
pub const SCOPES : [&str; 2] = ["scoped", "singleton"];