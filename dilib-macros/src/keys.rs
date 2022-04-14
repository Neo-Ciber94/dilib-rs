
/// `#[provide]`
pub const PROVIDE: &str = "provide";

/// `#[inject]` used for configuration
pub const INJECT : &str = "inject";

/// `#[provide(name="instance_name")]`
pub const NAME : &str = "name";

/// `#[provide(scope="...")]` `singleton` or `scoped` instance.
pub const SCOPE: &str = "scope";

/// `#[provide(bind="SomeTrait")]` trait to bind this instance to.
pub const BIND: &str = "bind";