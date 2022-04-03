use crate::provider::Provider;
use crate::scoped::Scoped;
use crate::{Injectable, InjectionKey, Resolved};
use std::any::TypeId;
use std::collections::hash_map::{Iter, Values};
use std::collections::HashMap;
use std::sync::Arc;

/// A convenient singleton type.
pub type Singleton<T> = Arc<T>;

// Insertion operation when adding new providers
#[derive(Debug, Eq, PartialEq)]
enum Operation {
    // Replace the provider if exist
    ReplaceIfExist,
    // Don't insert if the provider exist
    NoneIfExist,
}

/// Represents a store to register and retrieve objects.
#[derive(Default, Clone)]
pub struct Container<'a> {
    providers: HashMap<InjectionKey<'a>, Provider>,
}

impl<'a> Container<'a> {
    /// Constructs a new `Container`.
    pub fn new() -> Self {
        Container {
            providers: Default::default(),
        }
    }

    /// Adds a scoped factory function, and returns the previously registered
    /// provider for the given type, if any.
    #[inline]
    pub fn add_scoped<T, F>(&mut self, f: F) -> Option<Provider>
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        self.add_scoped_internal::<T>(Operation::ReplaceIfExist, Scoped::from_factory(f), None)
    }

    /// Adds a scoped factory function with a name, and returns the previously registered
    /// provider for the given type and name, if any.
    #[inline]
    pub fn add_scoped_with_name<T, F>(&mut self, name: &str, f: F) -> Option<Provider>
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        self.add_scoped_internal::<T>(
            Operation::ReplaceIfExist,
            Scoped::from_factory(f),
            Some(name),
        )
    }

    /// Adds a singleton, and returns the previously registered provider for the given type, if any.
    #[inline]
    pub fn add_singleton<T>(&mut self, value: T) -> Option<Provider>
    where
        T: 'static,
    {
        self.add_singleton_internal(Operation::ReplaceIfExist, None, value)
    }

    /// Adds a singleton with a name, and returns the previously registered provider
    /// for the given type and name, if any.
    #[inline]
    pub fn add_singleton_with_name<T>(&mut self, name: &str, value: T) -> Option<Provider>
    where
        T: 'static,
    {
        self.add_singleton_internal(Operation::ReplaceIfExist, Some(name), value)
    }

    /// Adds a scoped `Injectable` that depends on others providers,
    /// and returns the previously registered provider, if any.
    #[inline]
    pub fn add_deps<T>(&mut self) -> Option<Provider>
    where
        T: Injectable + 'static,
    {
        self.add_scoped_internal::<T>(
            Operation::ReplaceIfExist,
            Scoped::from_injectable(T::resolve),
            None,
        )
    }

    /// Adds a scoped named `Injectable` that depends on others providers,
    /// and returns the previously registered provider, if any.
    #[inline]
    pub fn add_deps_with_name<T>(&mut self, name: &str) -> Option<Provider>
    where
        T: Injectable + 'static,
    {
        self.add_scoped_internal::<T>(
            Operation::ReplaceIfExist,
            Scoped::from_injectable(T::resolve),
            Some(name),
        )
    }

    /// Attempts to add a scoped factory if there is no provider registered for the given type.
    #[inline]
    pub fn try_add_scoped<T, F>(&mut self, f: F)
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        self.add_scoped_internal::<T>(Operation::NoneIfExist, Scoped::from_factory(f), None);
    }

    /// Attempts to add a scoped factory if there is no provider registered for the given type and name.
    #[inline]
    pub fn try_add_scoped_with_name<T, F>(&mut self, name: &str, f: F)
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        self.add_scoped_internal::<T>(Operation::NoneIfExist, Scoped::from_factory(f), Some(name));
    }

    /// Attempts to add a singleton if there is no provider registered for the given type.
    #[inline]
    pub fn try_add_singleton<T>(&mut self, value: T)
    where
        T: 'static,
    {
        self.add_singleton_internal::<T>(Operation::NoneIfExist, None, value);
    }

    /// Attempts to add a singleton if there is nothing registered for the given type and name.
    #[inline]
    pub fn try_add_singleton_with_name<T>(&mut self, name: &str, value: T)
    where
        T: 'static,
    {
        self.add_singleton_internal::<T>(Operation::NoneIfExist, Some(name), value);
    }

    /// Attempts to add a scoped `Injectable` that depends on others providers
    /// if there is no provider register for the given type.
    #[inline]
    pub fn try_add_deps<T>(&mut self)
    where
        T: Injectable + 'static,
    {
        self.add_scoped_internal::<T>(
            Operation::NoneIfExist,
            Scoped::from_injectable(T::resolve),
            None,
        );
    }

    /// Attempts to add a scoped `Injectable` that depends on others providers
    /// if there is no provider register for the given type and name.
    #[inline]
    pub fn try_add_deps_with_name<T>(&mut self, name: &str)
    where
        T: Injectable + 'static,
    {
        self.add_scoped_internal::<T>(
            Operation::NoneIfExist,
            Scoped::from_injectable(T::resolve),
            Some(name),
        );
    }

    /// Returns a value registered for the given type or `None`
    /// if no provider is register for the given type.
    ///
    /// The returning value could be either scoped or a singleton.
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<Resolved<T>> {
        self.get_internal::<T>(None)
    }

    /// Returns a value registered for the given type and name or `None`
    /// if no provider is register for the given type.
    ///
    /// The returning value could be either scoped or a singleton.
    #[inline]
    pub fn get_with_name<T: 'static>(&self, name: &str) -> Option<Resolved<T>> {
        self.get_internal::<T>(Some(name))
    }

    fn get_internal<T>(&self, name: Option<&str>) -> Option<Resolved<T>>
    where
        T: 'static,
    {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, name);

        if let Some(provider) = self.providers.get(&key) {
            if provider.is_scoped() {
                match provider {
                    Provider::Scoped(f) => {
                        if f.is_factory() {
                            f.call_factory().map(Resolved::Scoped)
                        } else {
                            f.call_injectable(self).map(Resolved::Scoped)
                        }
                    }
                    Provider::Singleton(_) => unreachable!(),
                }
            } else {
                provider.get_singleton().map(Resolved::Singleton)
            }
        } else {
            None
        }
    }

    /// Returns a value registered for the given type, or `None`
    /// if no provider is register for the given type.
    #[inline]
    pub fn get_scoped<T>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.get_scoped_internal::<T>(None)
    }

    /// Returns a value registered for the given type and name, or `None`
    /// if no provider is register for the given type and name.
    #[inline]
    pub fn get_scoped_with_name<T>(&self, name: &str) -> Option<T>
    where
        T: 'static,
    {
        self.get_scoped_internal::<T>(Some(name))
    }

    /// Returns a singleton registered for the given type, or `None`
    /// if no provider is register for the given type.
    #[inline]
    pub fn get_singleton<T>(&self) -> Option<Singleton<T>>
    where
        T: 'static,
    {
        self.get_singleton_internal::<T>(None)
    }

    /// Returns a singleton registered for the given type and name, or `None`
    /// if no provider is register for the given type and name.
    #[inline]
    pub fn get_singleton_with_name<T>(&self, name: &str) -> Option<Singleton<T>>
    where
        T: 'static,
    {
        self.get_singleton_internal::<T>(Some(name))
    }

    /// Returns `true` if the `Container` have a provider for the given `InjectionKey`.
    pub fn contains(&self, key: InjectionKey) -> bool {
        self.providers.contains_key(&key)
    }

    /// Removes the provider with the given `InjectionKey` and returns it,
    /// or `None` if the provider is not found.
    pub fn remove(&mut self, key: InjectionKey<'a>) -> Option<Provider> {
        self.providers.remove(&key)
    }

    /// Returns the number of providers in this `Container`.
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns `true` is this container have no providers.
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Removes all the providers in this `Container`.
    pub fn clear(&mut self) {
        self.providers.clear();
    }

    /// Returns an iterator over the providers of this container.
    pub fn providers(&self) -> Values<'_, InjectionKey<'a>, Provider> {
        self.providers.values()
    }

    /// Returns an iterator over the keys and providers of this container.
    pub fn iter(&self) -> Iter<'_, InjectionKey<'a>, Provider> {
        self.providers.iter()
    }

    ////// Helper methods

    fn add_scoped_internal<T>(
        &mut self,
        op: Operation,
        scoped: Scoped,
        name: Option<&str>,
    ) -> Option<Provider>
    where
        T: 'static,
    {
        if op == Operation::NoneIfExist {
            let key = key_for::<T>(name);
            if self.contains(key) {
                return None;
            }
        }

        let name = name.map(|s| s.to_string());
        self.add_provider::<T>(Provider::Scoped(scoped), name)
    }

    fn add_singleton_internal<T>(
        &mut self,
        op: Operation,
        name: Option<&str>,
        value: T,
    ) -> Option<Provider>
    where
        T: 'static,
    {
        if op == Operation::NoneIfExist {
            let key = key_for::<T>(name);
            if self.contains(key) {
                return None;
            }
        }

        let singleton = Arc::new(value);
        let name = name.map(|s| s.to_string());
        self.add_provider::<T>(Provider::Singleton(singleton), name)
    }

    fn get_scoped_internal<T>(&self, name: Option<&str>) -> Option<T>
    where
        T: 'static,
    {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, name);
        match self.get_provider(key)? {
            Provider::Scoped(f) => {
                if f.is_factory() {
                    f.call_factory::<T>()
                } else {
                    f.call_injectable::<T>(self)
                }
            }
            Provider::Singleton(_) => unreachable!(),
        }
    }

    fn get_singleton_internal<T>(&self, name: Option<&str>) -> Option<Singleton<T>>
    where
        T: 'static,
    {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, name);
        self.get_provider(key).and_then(|p| p.get_singleton::<T>())
    }

    pub(crate) fn get_provider(&self, key: InjectionKey<'a>) -> Option<&Provider> {
        self.providers.get(&key)
    }

    pub(crate) fn add_provider<T: 'static>(
        &mut self,
        provider: Provider,
        name: Option<String>,
    ) -> Option<Provider> {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, name);
        self.providers.insert(key, provider)
    }
}

// Helper
#[inline(always)]
fn key_for<T: 'static>(name: Option<&str>) -> InjectionKey {
    let type_id = TypeId::of::<T>();
    InjectionKey::new(type_id, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn scoped_test() {
        let mut container = Container::new();
        container.add_scoped(|| "hello world"); // &str

        assert_eq!(container.len(), 1);

        let value = container.get_scoped::<&str>().unwrap();
        assert_eq!(value, "hello world");

        assert!(container.get_scoped::<String>().is_none());
    }

    #[test]
    fn scoped_with_name_test() {
        let mut container = Container::new();
        container.add_scoped_with_name("greet", || "hello world"); // &str

        assert_eq!(container.len(), 1);

        let value = container.get_scoped_with_name::<&str>("greet").unwrap();
        assert_eq!(value, "hello world");

        assert!(container.get_scoped_with_name::<String>("greet").is_none());
        assert!(container.get_scoped_with_name::<&str>("saludo").is_none());
    }

    #[test]
    fn singleton_test() {
        let mut container = Container::new();
        container.add_singleton(42069_i32);

        assert_eq!(container.len(), 1);

        let value = container.get_singleton::<i32>().unwrap();

        assert_eq!(*value, 42069_i32);
        assert!(container.get_singleton::<i64>().is_none());
    }

    #[test]
    fn singleton_with_name_test() {
        let mut container = Container::new();
        container.add_singleton_with_name("funny number", 42069_i32);

        assert_eq!(container.len(), 1);

        let value = container
            .get_singleton_with_name::<i32>("funny number")
            .unwrap();

        assert_eq!(*value, 42069_i32);
        assert!(container.get_singleton_with_name::<i32>("number").is_none());
    }

    #[test]
    fn contains_test() {
        let mut container = Container::new();
        container.add_scoped(|| 200_i32);
        container.add_scoped_with_name("number", || 999_i32);
        container.add_singleton(String::from("have a good day"));
        container.add_singleton_with_name("bye", "adios amigo");

        assert_eq!(container.len(), 4);

        assert!(container.contains(InjectionKey::of::<i32>()));
        assert!(container.contains(InjectionKey::with_name::<i32>("number")));
        assert!(container.contains(InjectionKey::of::<String>()));
        assert!(container.contains(InjectionKey::with_name::<&str>("bye")));
    }

    #[test]
    fn deps_test() {
        struct Greeter {
            message: String,
            total_greets: Singleton<Mutex<usize>>,
        }

        impl Greeter {
            fn greet(&self) -> String {
                *self.total_greets.lock().unwrap() += 1;
                self.message.clone()
            }
        }

        impl Injectable for Greeter {
            fn resolve(container: &Container) -> Self {
                let message = container.get_scoped_with_name::<String>("en_msg").unwrap();
                let total_greets = container
                    .get_singleton_with_name::<Mutex<usize>>("counter")
                    .unwrap();
                Greeter {
                    message,
                    total_greets,
                }
            }
        }

        let mut container = Container::new();
        container.add_singleton_with_name("counter", Mutex::new(0_usize));
        container.add_deps::<Greeter>();
        container.add_scoped_with_name("en_msg", || String::from("hello"));

        let greeter = container.get_scoped::<Greeter>().unwrap();
        let s = greeter.greet();
        assert_eq!(s.as_str(), "hello");

        greeter.greet();
        greeter.greet();

        assert_eq!(*greeter.total_greets.lock().unwrap(), 3);
    }

    #[test]
    fn deps_with_name_test() {
        struct Greeter {
            message: String,
            total_greets: Singleton<Mutex<usize>>,
        }

        impl Greeter {
            fn greet(&self) -> String {
                *self.total_greets.lock().unwrap() += 1;
                self.message.clone()
            }
        }

        impl Injectable for Greeter {
            fn resolve(container: &Container) -> Self {
                let message = container.get_scoped_with_name::<String>("en_msg").unwrap();
                let total_greets = container
                    .get_singleton_with_name::<Mutex<usize>>("counter")
                    .unwrap();
                Greeter {
                    message,
                    total_greets,
                }
            }
        }

        let mut container = Container::new();
        container.add_singleton_with_name("counter", Mutex::new(0_usize));
        container.add_deps_with_name::<Greeter>("en_greeter");
        container.add_scoped_with_name("en_msg", || String::from("hello"));

        let greeter = container
            .get_scoped_with_name::<Greeter>("en_greeter")
            .unwrap();
        assert!(container
            .get_scoped_with_name::<Greeter>("es_greeter")
            .is_none());

        let s = greeter.greet();
        assert_eq!(s.as_str(), "hello");

        greeter.greet();
        greeter.greet();

        assert_eq!(*greeter.total_greets.lock().unwrap(), 3);
    }

    #[test]
    fn try_add_scoped_test() {
        let mut container = Container::new();
        container.try_add_scoped(|| 100_i32);
        container.try_add_scoped(|| 300_i32);

        let value = container.get_scoped::<i32>().unwrap();
        assert_eq!(value, 100_i32);
    }

    #[test]
    fn try_add_scoped_with_name_test() {
        let mut container = Container::new();
        container.try_add_scoped_with_name("number", || 100_i32);
        container.try_add_scoped_with_name("number", || 300_i32);
        container.try_add_scoped_with_name("other_number", || 400_i32);

        let v1 = container.get_scoped_with_name::<i32>("number").unwrap();
        let v2 = container
            .get_scoped_with_name::<i32>("other_number")
            .unwrap();

        assert_eq!(v1, 100_i32);
        assert_eq!(v2, 400_i32);
    }

    #[test]
    fn try_add_singleton_test() {
        let mut container = Container::new();
        container.try_add_singleton("hello world");
        container.try_add_singleton("hola mundo");

        let value = container.get_singleton::<&str>().unwrap();
        assert_eq!(*value, "hello world");
    }

    #[test]
    fn try_add_singleton_with_name_test() {
        let mut container = Container::new();
        container.try_add_singleton_with_name("greeting", "hello world");
        container.try_add_singleton_with_name("greeting", "hola mundo");
        container.try_add_singleton_with_name("goodbye", "bye world");

        let v1 = container
            .get_singleton_with_name::<&str>("greeting")
            .unwrap();
        let v2 = container
            .get_singleton_with_name::<&str>("goodbye")
            .unwrap();

        assert_eq!(*v1, "hello world");
        assert_eq!(*v2, "bye world");
    }

    #[test]
    fn try_add_deps_test() {
        struct IntWrapper(i32);

        impl Injectable for IntWrapper {
            fn resolve(container: &Container) -> Self {
                let value = container.get_scoped::<i32>().unwrap();
                IntWrapper(value)
            }
        }

        let mut container = Container::new();
        container.try_add_deps::<IntWrapper>();

        let provider = container.add_deps::<IntWrapper>();
        assert!(provider.is_some());

        container.add_scoped(|| 22_i32);

        let wrapper = container.get_scoped::<IntWrapper>().unwrap();
        assert_eq!(22_i32, wrapper.0);
    }

    #[test]
    fn try_add_deps_with_name_test() {
        struct IntWrapper(i32);

        impl Injectable for IntWrapper {
            fn resolve(container: &Container) -> Self {
                let value = container.get_scoped::<i32>().unwrap();
                IntWrapper(value)
            }
        }

        let mut container = Container::new();
        container.try_add_deps_with_name::<IntWrapper>("wrapper");

        let p1 = container.add_deps_with_name::<IntWrapper>("wrapper");
        let p2 = container.add_deps_with_name::<IntWrapper>("nothing");

        assert!(p1.is_some());
        assert!(p2.is_none());

        container.add_scoped(|| 22_i32);

        let w1 = container
            .get_scoped_with_name::<IntWrapper>("wrapper")
            .unwrap();
        let w2 = container
            .get_scoped_with_name::<IntWrapper>("nothing")
            .unwrap();
        assert_eq!(22_i32, w1.0);
        assert_eq!(22_i32, w2.0);
    }

    #[test]
    fn remove_test() {
        let mut container = Container::new();
        assert_eq!(container.len(), 0);

        container.add_scoped(|| true);
        container.add_singleton(String::from("blue"));
        container.add_scoped_with_name("number", || 200_i32);
        container.add_singleton_with_name("color", String::from("red"));

        assert_eq!(container.len(), 4);

        assert!(container.remove(InjectionKey::of::<bool>()).is_some());

        // Provider already removed
        assert!(container.remove(InjectionKey::of::<bool>()).is_none());

        assert_eq!(container.len(), 3);

        // Provider is of incorrect kind
        assert!(container.remove(InjectionKey::of::<String>()).is_some());

        assert_eq!(container.len(), 2);

        assert!(container
            .remove(InjectionKey::with_name::<i32>("number"))
            .is_some());
        assert!(container
            .remove(InjectionKey::with_name::<String>("color"))
            .is_some());
        assert_eq!(container.len(), 0);
    }

    #[test]
    fn clear_test() {
        let mut container = Container::new();
        assert_eq!(container.len(), 0);

        container.add_scoped(|| true);
        container.add_singleton(String::from("blue"));

        assert_eq!(container.len(), 2);

        container.clear();
        assert_eq!(container.len(), 0);
    }

    #[test]
    fn providers_test() {
        let mut container = Container::new();
        container.add_scoped(|| true);
        container.add_singleton(0.25_f32);
        container.add_scoped(|| 200_usize);

        let providers = container.providers();
        assert_eq!(3, providers.clone().count());

        let v1 = providers
            .clone()
            .filter_map(|p| p.get_scoped::<bool>())
            .last();

        let v2 = providers
            .clone()
            .filter_map(|p| p.get_singleton::<f32>())
            .last();

        let v3 = providers
            .clone()
            .filter_map(|p| p.get_scoped::<usize>())
            .last();

        assert_eq!(Some(true), v1);
        assert_eq!(0.25_f32, *v2.unwrap());
        assert_eq!(Some(200_usize), v3);
    }

    #[test]
    fn iter_test() {
        let mut container = Container::new();
        container.add_scoped_with_name("truthfulness", || true);
        container.add_singleton(2500_i32);

        let iter = container.iter();
        assert_eq!(2, iter.clone().count());

        let (k1, p1) = iter
            .clone()
            .filter(|(k, _)| k.type_id() == TypeId::of::<bool>())
            .last()
            .unwrap();

        let (k2, p2) = iter
            .clone()
            .filter(|(k, _)| k.type_id() == TypeId::of::<i32>())
            .last()
            .unwrap();

        assert_eq!(Some("truthfulness"), k1.name());
        assert_eq!(TypeId::of::<bool>(), k1.type_id());
        assert_eq!(Some(true), p1.get_scoped::<bool>());

        assert_eq!(None, k2.name());
        assert_eq!(TypeId::of::<i32>(), k2.type_id());
        assert_eq!(2500_i32, *p2.get_singleton::<i32>().unwrap());
    }
}
