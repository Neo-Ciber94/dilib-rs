/// Helper macro to bind a `trait` to it's implementation in a `Container` as scoped.
///
/// # Usage
/// `register_scoped_trait!(container, name, trait, implementation)`
///
/// - `container`: identifier of the container to add the implementation.
/// - `name`: optional name to store the provider.
/// - `trait`: the type of the trait.
/// - `implementation`: the implementation of the trait. This can use `{ implementation }` brackets.
///
/// # Examples
/// ## Basic usage
/// ```rust
/// #[macro_use]
/// extern crate dilib;
/// use dilib::*;
///
/// trait Greet {
///    fn greet(&self) -> &str;
/// }
///
/// struct EnglishGreeting;
/// impl Greet for EnglishGreeting {
///     fn greet(&self) -> &str {
///         "Hello"
///     }
/// }
/// fn main () {
///     let mut container = Container::new();
///     register_scoped_trait!(container, Greet, EnglishGreeting);
///
///     let greeting = get_scoped_trait!(container, Greet).unwrap();
///     assert_eq!(greeting.greet(), "Hello");
/// }
/// ```
///
/// ## With named trait
/// ```rust
/// #[macro_use]
/// extern crate dilib;
/// use dilib::*;
///
/// trait Greet {
///     fn greet(&self) -> &'static str;
/// }
///
/// struct Hello;
/// impl Greet for Hello {
///     fn greet(&self) -> &'static str {
///         "hello world"
///     }
/// }
///
/// struct Bye;
/// impl Greet for Bye {
///     fn greet(&self) -> &'static str {
///         "bye world"
///     }
/// }
///
/// fn main() {
///     let mut container = Container::new();
///     register_scoped_trait!(container, "hello", Greet, Hello);
///     register_scoped_trait!(container, "bye", Greet, { Bye });
///
///     // Returns a `Box<dyn Greet>`
///     let hello = get_scoped_trait!(container, Greet, "hello").unwrap();
///     let bye = get_scoped_trait!(container, Greet, "bye").unwrap();
///
///     assert_eq!(hello.greet(), "hello world");
///     assert_eq!(bye.greet(), "bye world");
/// }
/// ```
#[macro_export]
macro_rules! register_scoped_trait {
    ($container:ident, $trait_type:ident, $impl_expr:expr) => {{
        $container.add_scoped(|| -> std::boxed::Box<dyn $trait_type> {
            let ret: std::boxed::Box<dyn $trait_type> = {
                let value = $impl_expr;
                std::boxed::Box::new(value)
            };

            ret
        })
    }};

    ($container:ident, $name:literal, $trait_type:ident, $impl_expr:expr) => {{
        $container.add_scoped_with_name($name, || -> std::boxed::Box<dyn $trait_type> {
            let ret: std::boxed::Box<dyn $trait_type> = {
                let value = $impl_expr;
                std::boxed::Box::new(value)
            };

            ret
        })
    }};

    ($container:ident, $trait_type:ident, { $impl_expr:expr }) => {{
        $crate::register_scoped_trait!($container, $trait_type, $impl_expr);
    }};

    ($container:ident, $name:literal, $trait_type:ident, { $impl_expr:expr }) => {{
        $crate::register_scoped_trait!($container, $trait_type, $name, $impl_expr);
    }};
}

/// Helper macro to get the implementation of a `trait` in a `Container` as scoped.
///
/// # Usage
/// `get_scoped_trait!(container, trait, name)`
///
/// - `container`: the container to get the implementation of the trait.
/// - `trait`: the trait to get the implementation from.
/// - `name`: optional name of the implementation.
#[macro_export]
macro_rules! get_scoped_trait {
    ($container:ident, $trait_type:ident) => {{
        let ret: std::option::Option<Box<dyn $trait_type>> = $container.get_scoped();
        ret
    }};

    ($container:ident, $trait_type:ident, $name:literal) => {{
        let ret: std::option::Option<Box<dyn $trait_type>> = $container.get_scoped_with_name($name);
        ret
    }};
}

/// Helper macro to bind a `trait` to it's implementation in a `Container` as a singleton.
///
/// # Usage
/// `register_singleton_trait!(container, name, trait, implementation)`
///
/// - `container`: identifier of the container to add the implementation.
/// - `name`: optional name to store the provider.
/// - `trait`: the type of the trait.
/// - `implementation`: the implementation of the trait. This can use `{ implementation }` brackets.
///
/// # Examples
///
/// ## Basic usage
/// ```rust
/// #[macro_use]
/// extern crate dilib;
/// use dilib::*;
///
/// trait Greet {
///    fn greet(&self) -> &str;
/// }
///
/// struct HelloWorld;
/// impl Greet for HelloWorld {
///   fn greet(&self) -> &'static str {
///         "hello world"
///     }
/// }
///
/// fn main() {
///     let mut container = Container::new();
///     register_singleton_trait!(container, Greet, HelloWorld);
///
///     let greet = get_singleton_trait!(container, Greet).unwrap();
///     assert_eq!(greet.greet(), "hello world");
/// }
/// ```
///
/// ## With named trait
/// ```rust
/// #[macro_use]
/// extern crate dilib;
/// use dilib::macros::*;
/// use dilib::Container;
///
/// trait BinaryOp {
///     fn calc(&self, lhs: i32, rhs: i32) -> i32;
/// }
///
/// struct Sum;
/// struct Prod;
///
/// impl BinaryOp for Sum {
///     fn calc(&self, lhs: i32, rhs: i32) -> i32 { lhs + rhs }
/// }
///
/// impl BinaryOp for Prod {
///     fn calc(&self, lhs: i32, rhs: i32) -> i32 { lhs * rhs }
/// }
///
/// fn main() {
///     let mut container = Container::new();
///     let c = register_singleton_trait!(container, "sum", BinaryOp, Sum);
///     register_singleton_trait!(container, "prod", BinaryOp, Prod);
///
///     let sum = get_singleton_trait!(container, BinaryOp, "sum").unwrap();
///     let prod = get_singleton_trait!(container, BinaryOp, "prod").unwrap();
///
///     assert_eq!(5, sum.calc(2, 3));
///     assert_eq!(6, prod.calc(3, 2));
/// }
/// ```
#[macro_export]
macro_rules! register_singleton_trait {
    ($container:ident, $trait_type:ident, $impl_expr:expr) => {{
        type SafeTrait = dyn $trait_type + Send + Sync;
        let x : std::boxed::Box<SafeTrait> = Box::new($impl_expr);
        $container.add_singleton::<std::boxed::Box<SafeTrait>>(x)
    }};

    ($container:ident, $name:literal, $trait_type:ident, $impl_expr:expr) => {{
        type SafeTrait = dyn $trait_type + Send + Sync;
        let x : std::boxed::Box<SafeTrait> = Box::new($impl_expr);
        $container.add_singleton_with_name::<std::boxed::Box<SafeTrait>>($name, x)
    }};

    ($container:ident, $trait_type:ident, { $impl_expr:expr }) => {{
        $crate::register_singleton_trait!($container, $trait_type, $impl_expr);
    }};

    ($container:ident, $name:literal, $trait_type:ident, { $impl_expr:expr }) => {{
        $crate::register_singleton_trait!($container, $trait_type, $name:literal, $impl_expr);
    }};
}

/// Helper macro to get the implementation of a `trait` in a `Container` as a singleton.
///
/// # Usage
/// `get_singleton_trait!(container, trait, name)`
///
/// - `container`: the container to get the implementation of the trait.
/// - `trait`: the trait to get the implementation from.
/// - `name`: optional name of the implementation.
#[macro_export]
macro_rules! get_singleton_trait {
    ($container:ident, $trait_type:ident) => {{
        type SafeTrait = dyn $trait_type + Send + Sync;
        let ret = $container.get_singleton::<std::boxed::Box<SafeTrait>>();
        ret
    }};

    ($container:ident, $trait_type:ident, $name:literal) => {{
        type SafeTrait = dyn $trait_type + Send + Sync;
        let ret = $container.get_singleton_with_name::<std::boxed::Box<SafeTrait>>($name);
        ret
    }};
}