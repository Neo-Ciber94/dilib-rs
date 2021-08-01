/// Helper macro to bind a `trait` with it's implementation in a `Container`.
///
/// # Usage
/// `register_scoped_trait!(container, trait, name, implementation)`
///
/// - `container`: identifier of the container to add the implementation.
/// - `trait`: the type of the trait.
/// - `name`: optional name to store the provider.
/// - `implementation`: the implementation of the trait. This can use `{ implementation }` brackets.
///
/// # Example
/// ```ignore
/// use dilib::Container;
/// use dilib::macros::*;
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
/// let mut container = Container::new();
/// register_scoped_trait!(container, Greet, "hello", Hello);
/// register_scoped_trait!(container, Greet, "bye", { Bye });
///
/// // Returns a `Box<dyn Greet>`
/// let hello = get_scoped_trait!(container, Greet, "hello");
/// let bye = get_scoped_trait!(container, Greet, "bye");
///
/// assert!(hello.greet(), "hello world");
/// assert!(bye.greet(), "bye world");
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

    ($container:ident, $trait_type:ident, $name:literal, $impl_expr:expr) => {{
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

    ($container:ident, $trait_type:ident, $name:literal, { $impl_expr:expr }) => {{
        $crate::register_scoped_trait!($container, $trait_type, $name, $impl_expr);
    }};
}

/// Helper macro to get the implementation of a `trait` in a `Container`.
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