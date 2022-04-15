/// Helper macro to bind a `trait` to it's implementation in a `Container` as scoped.
///
/// # Overloads
/// - `add_scoped_trait!(container, name, trait => implementation)`
/// - `add_scoped_trait!(container, trait => implementation)`
/// - `add_scoped_trait!(container, name, trait @ Inject)`
/// - `add_scoped_trait!(container, trait @ Inject)`
///
/// # Params
/// - `container`: identifier of the container to add the implementation.
/// - `name`: optional name to store the provider.
/// - `trait`: the type of the trait.
/// - `implementation`: the implementation of the trait.
/// - `Inject`: a struct that implements [`Inject`].
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
///     add_scoped_trait!(container, Greet => EnglishGreeting).unwrap();
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
///     add_scoped_trait!(container, "hello", Greet => Hello).unwrap();
///     add_scoped_trait!(container, "bye", Greet => { Bye }).unwrap();
///
///     // Returns a `Box<dyn Greet>`
///     let hello = get_scoped_trait!(container, Greet, "hello").unwrap();
///     let bye = get_scoped_trait!(container, Greet, "bye").unwrap();
///
///     assert_eq!(hello.greet(), "hello world");
///     assert_eq!(bye.greet(), "bye world");
/// }
/// ```
///
/// [`Inject`]: crate::Inject
#[macro_export]
macro_rules! add_scoped_trait {
    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)? => $impl_expr:expr) => {{
        $container.add_scoped(|| -> std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> {
            let ret: std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> = {
                let value = $impl_expr;
                std::boxed::Box::new(value)
            };

            ret
        })
    }};

    ($container:ident, $name:literal, $trait_type:ident $(<$($generic:ident),+>)? => $impl_expr:expr) => {{
        $container.add_scoped_with_name($name, || -> std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> {
            let ret: std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> = {
                let value = $impl_expr;
                std::boxed::Box::new(value)
            };

            ret
        })
    }};

    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)? => { $impl_expr:expr }) => {{
        $crate::register_scoped_trait!($container, $trait_type $(<$($generic),+>)?, $impl_expr);
    }};

    ($container:ident, $name:literal, $trait_type:ident $(<$($generic:ident),+>)? => { $impl_expr:expr }) => {{
        $crate::register_scoped_trait!($container, $trait_type $(<$($generic),+>)?, $name, $impl_expr);
    }};

    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)? @ $inject_type:ty) => {{
        $container.add_deps_fn(|container| -> std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> {
            let ret: std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> = {
                let value = <$inject_type>::inject(container);
                std::boxed::Box::new(value)
            };

            ret
        })
    }};

     ($container:ident,  $name:literal,, $trait_type:ident $(<$($generic:ident),+>)? @ $inject_type:ty) => {{
        $container.add_deps_fn_with_name($name, |container| -> std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> {
            let ret: std::boxed::Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static> = {
                let value = <$inject_type>::inject(container);
                std::boxed::Box::new(value)
            };

            ret
        })
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
    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)?) => {{
        let ret: std::option::Option<Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static>> = $container.get_scoped();
        ret
    }};

    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)?, $name:literal) => {{
        let ret: std::option::Option<Box<dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static>> = $container.get_scoped_with_name($name);
        ret
    }};
}

/// Helper macro to bind a `trait` to it's implementation in a `Container` as a singleton.
///
/// # Overloads
/// `add_singleton_trait!(container, name, trait => implementation)`
/// `add_singleton_trait!(container, trait => implementation)`
/// `add_singleton_trait!(container, name, trait @ Inject)`
/// `add_singleton_trait!(container, trait @ Inject)`
///
/// # Params
/// - `container`: identifier of the container to add the implementation.
/// - `name`: optional name to store the provider.
/// - `trait`: the type of the trait.
/// - `implementation`: the implementation of the trait.
/// - `Inject`: a struct that implements [`Inject`].
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
///     add_singleton_trait!(container, Greet => HelloWorld).unwrap();
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
/// use dilib::*;
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
///     add_singleton_trait!(container, "sum", BinaryOp => Sum).unwrap();
///     add_singleton_trait!(container, "prod", BinaryOp => Prod).unwrap();
///
///     let sum = get_singleton_trait!(container, BinaryOp, "sum").unwrap();
///     let prod = get_singleton_trait!(container, BinaryOp, "prod").unwrap();
///
///     assert_eq!(5, sum.calc(2, 3));
///     assert_eq!(6, prod.calc(3, 2));
/// }
/// ```
///
/// [`Inject`]: crate::Inject
#[macro_export]
macro_rules! add_singleton_trait {
    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)? => $impl_expr:expr) => {{
        type SafeTrait = dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static;
        let x: std::boxed::Box<SafeTrait> = Box::new($impl_expr);
        $container.add_singleton::<std::boxed::Box<SafeTrait>>(x)
    }};

    ($container:ident, $name:literal, $trait_type:ident $(<$($generic:ident),+>)? => $impl_expr:expr) => {{
        type SafeTrait = dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static;
        let x: std::boxed::Box<SafeTrait> = Box::new($impl_expr);
        $container.add_singleton_with_name::<std::boxed::Box<SafeTrait>>($name, x)
    }};

    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)? => { $impl_expr:expr }) => {{
        $crate::register_singleton_trait!($container, $trait_type $(<$($generic),+>)?, $impl_expr);
    }};

    ($container:ident, $name:literal, $trait_type:ident $(<$($generic:ident),+>)? => { $impl_expr:expr }) => {{
        $crate::register_singleton_trait!($container, $trait_type $(<$($generic),+>)?, $name: literal, $impl_expr);
    }};

    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)? @ $inject_type:ty) => {{
        #[cfg(feature = "lazy")]
        {
            type SafeTrait = dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static;
            $container.add_lazy_singleton::<std::boxed::Box<SafeTrait>>(|container| {
                let ret: std::boxed::Box<SafeTrait> = Box::new($inject_type);
                ret
            })
        }

        #cfg(not(feature="lazy"))
        panic!("`lazy` feature is not enabled. Please enable it to use this feature.");
    }};

    ($container:ident, $name:literal, $trait_type:ident $(<$($generic:ident),+>)? @ $inject_type:ty) => {{
        #[cfg(feature = "lazy")]
        {
            type SafeTrait = dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static;
            $container.add_lazy_singleton_with_name::<std::boxed::Box<SafeTrait>>($name, |container| {
                let ret: std::boxed::Box<SafeTrait> = Box::new($inject_type);
                ret
            })
        }

        #cfg(not(feature="lazy"))
        panic!("`lazy` feature is not enabled. Please enable it to use this feature.");
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
    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)?) => {{
        let ret = $container.get_singleton::<std::boxed::Box<(dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static)>>();
        ret
    }};

    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)?, $name:literal) => {{
        let ret = $container.get_singleton_with_name::<std::boxed::Box<(dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static)>>($name);
        ret
    }};
}

/// Helper macro to get an implementation of a `trait` in a `Container`.
#[macro_export]
macro_rules! get_resolved_trait {
    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)?) => {{
        let ret = $container.get::<std::boxed::Box<(dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static)>>();
        ret
    }};

    ($container:ident, $trait_type:ident $(<$($generic:ident),+>)?, $name:literal) => {{
        let ret = $container.get_with_name::<std::boxed::Box<(dyn $trait_type $(<$($generic),+>)? + Send + Sync + 'static)>>($name);
        ret
    }};
}

#[cfg(test)]
mod tests {
    use crate::Container;

    #[test]
    fn compile_get_scoped_trait_test_1() {
        let mut container = Container::new();
        add_scoped_trait!(container, Gen1<i32> => Gen1Impl::<i32>(10)).unwrap();

        let _ret = get_scoped_trait!(container, Gen1<i32>).unwrap();
    }

    #[test]
    fn compile_get_scoped_trait_test_2() {
        let mut container = Container::new();
        add_scoped_trait!(container, Gen2<i32, bool> => Gen2Impl::<i32, bool>(10, false))
            .unwrap();

        let _ret = get_scoped_trait!(container, Gen2<i32, bool>).unwrap();
    }

    #[test]
    fn compile_get_scoped_trait_test_3() {
        let mut container = Container::new();
        add_scoped_trait!(container, Gen3<i32, bool, String> => Gen3Impl::<i32, bool, String>(10, false, String::from("test"))).unwrap();

        let _ret = get_scoped_trait!(container, Gen3<i32, bool, String>).unwrap();
    }

    #[test]
    fn compile_get_resolved_trait() {
        let mut container = Container::new();
        add_scoped_trait!(container, Gen1<i32> => Gen1Impl::<i32>(10)).unwrap();
        add_singleton_trait!(container, Gen2<i32, bool> => Gen2Impl::<i32, bool>(10, false))
            .unwrap();

        let _r1 = get_resolved_trait!(container, Gen1<i32>).unwrap();
        let _r2 = get_resolved_trait!(container, Gen2<i32, bool>).unwrap();
    }

    trait Gen1<T1> {
        fn get(&self) -> &T1;
    }

    trait Gen2<T1, T2> {
        fn get(&self) -> (&T1, &T2);
    }

    trait Gen3<T1, T2, T3> {
        fn get(&self) -> (&T1, &T2, &T3);
    }

    struct Gen1Impl<T1>(T1);
    impl<T1> Gen1<T1> for Gen1Impl<T1> {
        fn get(&self) -> &T1 {
            &self.0
        }
    }

    struct Gen2Impl<T1, T2>(T1, T2);
    impl<T1, T2> Gen2<T1, T2> for Gen2Impl<T1, T2> {
        fn get(&self) -> (&T1, &T2) {
            (&self.0, &self.1)
        }
    }

    struct Gen3Impl<T1, T2, T3>(T1, T2, T3);
    impl<T1, T2, T3> Gen3<T1, T2, T3> for Gen3Impl<T1, T2, T3> {
        fn get(&self) -> (&T1, &T2, &T3) {
            (&self.0, &self.1, &self.2)
        }
    }
}
