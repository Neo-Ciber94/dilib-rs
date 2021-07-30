use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::provider::{Provider, ProviderKind};
use crate::scoped::Scoped;
use crate::{Injectable, InjectionKey};
use std::collections::hash_map::{Iter, Values};

/// A convenient singleton type.
pub type Singleton<T> = Arc<Mutex<T>>;

/// Represents a container for providers.
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

    /// Adds a scoped factory function.
    pub fn add_scoped<T, F>(&mut self, f: F) -> Option<Provider>
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        self.add_scoped_with_name(None, f)
    }

    /// Adds a scoped factory function with a name.
    pub fn add_scoped_with_name<T, F>(&mut self, name: Option<&str>, f: F) -> Option<Provider>
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        let scoped = Scoped::from_factory(f);
        let name = name.map(|s| s.to_string());
        self.add_provider::<T>(Provider::Scoped(scoped), name)
    }

    /// Adds a singleton.
    pub fn add_singleton<T>(&mut self, value: T) -> Option<Provider>
    where
        T: Send + Sync + 'static,
    {
        self.add_singleton_with_name(None, value)
    }

    /// Adds a singleton with a name.
    pub fn add_singleton_with_name<T>(&mut self, name: Option<&str>, value: T) -> Option<Provider>
    where
        T: Send + Sync + 'static,
    {
        let singleton = Arc::new(Mutex::new(value));
        let name = name.map(|s| s.to_string());
        self.add_provider::<T>(Provider::Singleton(singleton), name)
    }

    /// Adds an scoped injectable type.
    pub fn add_deps<T>(&mut self) -> Option<Provider>
    where
        T: Injectable + 'static,
    {
        self.add_deps_with_name::<T>(None)
    }

    /// Adds an scoped injectable type with a name.
    pub fn add_deps_with_name<T>(&mut self, name: Option<&str>) -> Option<Provider>
    where
        T: Injectable + 'static,
    {
        let scoped = Scoped::from_injectable(T::resolve);
        let name = name.map(|s| s.to_string());
        self.add_provider::<T>(Provider::Scoped(scoped), name)
    }

    /// Attempts to add a scoped factory or fails if already exists a provider.
    pub fn try_add_scoped<T, F>(&mut self, f: F)
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        self.try_add_scoped_with_name::<T, F>(None, f)
    }

    /// Attempts to add a scoped factory with a name or fails if already exists a provider.
    pub fn try_add_scoped_with_name<T, F>(&mut self, name: Option<&str>, f: F)
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        let key = key_for::<T>(name, ProviderKind::Scoped);
        if !self.contains(key) {
            self.add_scoped_with_name(name, f);
        }
    }

    /// Attempts to add a singleton or fails if already exists a provider.
    pub fn try_add_singleton<T>(&mut self, value: T)
    where
        T: Send + Sync + 'static,
    {
        self.try_add_singleton_with_name::<T>(None, value)
    }

    /// Attempts to add a singleton with a name or fails if already exists a provider.
    pub fn try_add_singleton_with_name<T>(&mut self, name: Option<&str>, value: T)
    where
        T: Send + Sync + 'static,
    {
        let key = key_for::<T>(name, ProviderKind::Singleton);
        if !self.contains(key) {
            self.add_singleton_with_name(name, value);
        }
    }

    /// Attempts to add an scoped injectable type or fails if already exists a provider.
    pub fn try_add_deps<T>(&mut self)
    where
        T: Injectable + 'static,
    {
        self.try_add_deps_with_name::<T>(None)
    }

    /// Attempts to add an scoped injectable type with a name or fails if already exists a provider.
    pub fn try_add_deps_with_name<T>(&mut self, name: Option<&str>)
    where
        T: Injectable + 'static,
    {
        let key = key_for::<T>(name, ProviderKind::Scoped);
        if !self.contains(key) {
            self.add_deps_with_name::<T>(name);
        }
    }

    /// Gets a value of the specified type `T` or `None`
    /// if there is no provider for the given type.
    pub fn get_scoped<T>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.get_scoped_internal::<T>(None)
    }

    /// Gets a value of the specified type `T` and the given name or `None`
    /// if there is no provider for the given type and/or name.
    pub fn get_scoped_with_name<T>(&self, name: &str) -> Option<T>
    where
        T: 'static,
    {
        self.get_scoped_internal::<T>(Some(name))
    }

    /// Gets the singleton value of the specified type or `None`
    /// if there is no provider for the given type.
    pub fn get_singleton<T>(&self) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
    {
        self.get_singleton_internal::<T>(None)
    }

    /// Gets a singleton value of the specified type and the given name or `None`
    /// if there is no provider for the given type and/or name.
    pub fn get_singleton_with_name<T>(&self, name: &str) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
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

    /// Removes all the providers in this `Container.
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

    ////// Private methods

    fn get_provider(&self, key: InjectionKey<'a>) -> Option<&Provider> {
        self.providers.get(&key)
    }

    fn add_provider<T: 'static>(
        &mut self,
        provider: Provider,
        name: Option<String>,
    ) -> Option<Provider> {
        let kind = provider.kind();
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, kind, name);
        self.providers.insert(key, provider)
    }

    fn get_scoped_internal<T>(&self, name: Option<&str>) -> Option<T>
    where
        T: 'static,
    {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, ProviderKind::Scoped, name);

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
        T: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, ProviderKind::Singleton, name);
        self.get_provider(key)
            .map(|p| p.get_singleton::<T>())
            .flatten()
    }
}

// Helper
#[inline]
fn key_for<T: 'static>(name: Option<&str>, kind: ProviderKind) -> InjectionKey {
    let type_id = TypeId::of::<T>();
    InjectionKey::new(type_id, kind, name)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        container.add_scoped_with_name(Some("greet"), || "hello world"); // &str

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

        assert_eq!(*value.lock().unwrap(), 42069_i32);
        assert!(container.get_singleton::<i64>().is_none());
    }

    #[test]
    fn singleton_with_name_test() {
        let mut container = Container::new();
        container.add_singleton_with_name(Some("funny number"), 42069_i32);

        assert_eq!(container.len(), 1);

        let value = container
            .get_singleton_with_name::<i32>("funny number")
            .unwrap();

        assert_eq!(*value.lock().unwrap(), 42069_i32);
        assert!(container.get_singleton_with_name::<i32>("number").is_none());
    }

    #[test]
    fn contains_test() {
        let mut container = Container::new();
        container.add_scoped(|| 200_i32);
        container.add_scoped_with_name(Some("number"), || 999_i32);
        container.add_singleton(String::from("have a good day"));
        container.add_singleton_with_name(Some("bye"), "adios amigo");

        assert_eq!(container.len(), 4);

        assert!(container.contains(InjectionKey::of::<i32>(ProviderKind::Scoped)));
        assert!(container.contains(InjectionKey::with_name::<i32>(
            ProviderKind::Scoped,
            "number"
        )));
        assert!(container.contains(InjectionKey::of::<String>(ProviderKind::Singleton)));
        assert!(container.contains(InjectionKey::with_name::<&str>(
            ProviderKind::Singleton,
            "bye"
        )));
    }

    #[test]
    fn deps_test() {
        struct Greeter {
            message: String,
            total_greets: Singleton<usize>,
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
                    .get_singleton_with_name::<usize>("counter")
                    .unwrap();
                Greeter {
                    message,
                    total_greets,
                }
            }
        }

        let mut container = Container::new();
        container.add_singleton_with_name(Some("counter"), 0_usize);
        container.add_deps::<Greeter>();
        container.add_scoped_with_name(Some("en_msg"), || String::from("hello"));

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
            total_greets: Singleton<usize>,
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
                    .get_singleton_with_name::<usize>("counter")
                    .unwrap();
                Greeter {
                    message,
                    total_greets,
                }
            }
        }

        let mut container = Container::new();
        container.add_singleton_with_name(Some("counter"), 0_usize);
        container.add_deps_with_name::<Greeter>(Some("en_greeter"));
        container.add_scoped_with_name(Some("en_msg"), || String::from("hello"));

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
        container.try_add_scoped_with_name(Some("number"), || 100_i32);
        container.try_add_scoped_with_name(Some("number"), || 300_i32);
        container.try_add_scoped_with_name(Some("other_number"), || 400_i32);

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
        assert_eq!(*value.lock().unwrap(), "hello world");
    }

    #[test]
    fn try_add_singleton_with_name_test() {
        let mut container = Container::new();
        container.try_add_singleton_with_name(Some("greeting"), "hello world");
        container.try_add_singleton_with_name(Some("greeting"), "hola mundo");
        container.try_add_singleton_with_name(Some("goodbye"), "bye world");

        let v1 = container
            .get_singleton_with_name::<&str>("greeting")
            .unwrap();
        let v2 = container
            .get_singleton_with_name::<&str>("goodbye")
            .unwrap();

        assert_eq!(*v1.lock().unwrap(), "hello world");
        assert_eq!(*v2.lock().unwrap(), "bye world");
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
        container.try_add_deps_with_name::<IntWrapper>(Some("wrapper"));

        let p1 = container.add_deps_with_name::<IntWrapper>(Some("wrapper"));
        let p2 = container.add_deps_with_name::<IntWrapper>(Some("nothing"));

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
        container.add_scoped_with_name(Some("number"), || 200_i32);
        container.add_singleton_with_name(Some("color"), String::from("red"));

        assert_eq!(container.len(), 4);

        assert!(container
            .remove(InjectionKey::of::<bool>(ProviderKind::Scoped))
            .is_some());

        // Provider already removed
        assert!(container
            .remove(InjectionKey::of::<bool>(ProviderKind::Scoped))
            .is_none());

        assert_eq!(container.len(), 3);

        // Provider is of incorrect kind
        assert!(container
            .remove(InjectionKey::of::<String>(ProviderKind::Scoped))
            .is_none());

        assert!(container
            .remove(InjectionKey::of::<String>(ProviderKind::Singleton))
            .is_some());

        assert_eq!(container.len(), 2);

        assert!(container
            .remove(InjectionKey::with_name::<i32>(
                ProviderKind::Scoped,
                "number"
            ))
            .is_some());
        assert!(container
            .remove(InjectionKey::with_name::<String>(
                ProviderKind::Singleton,
                "color"
            ))
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

        let v1 = providers.clone()
            .filter_map(|p| p.get_scoped::<bool>())
            .last();

        let v2 = providers.clone()
            .filter_map(|p| p.get_singleton::<f32>())
            .last();

        let v3 = providers.clone()
            .filter_map(|p| p.get_scoped::<usize>())
            .last();

        assert_eq!(Some(true), v1);
        assert_eq!(0.25_f32, *v2.unwrap().lock().expect("unable to get singleton"));
        assert_eq!(Some(200_usize), v3);
    }

    #[test]
    fn iter_test() {
        let mut container = Container::new();
        container.add_scoped_with_name(Some("truthfulness"), || true);
        container.add_singleton(2500_i32);

        let iter = container.iter();
        assert_eq!(2, iter.clone().count());

        let (k1, p1) = iter.clone()
            .filter(|(k, _)| k.type_id() == TypeId::of::<bool>())
            .last()
            .unwrap();

        let (k2, p2) = iter.clone()
            .filter(|(k, _)| k.type_id() == TypeId::of::<i32>())
            .last()
            .unwrap();

        assert_eq!(Some("truthfulness"), k1.name());
        assert_eq!(ProviderKind::Scoped, k1.kind());
        assert_eq!(TypeId::of::<bool>(), k1.type_id());
        assert_eq!(Some(true), p1.get_scoped::<bool>());

        assert_eq!(None, k2.name());
        assert_eq!(ProviderKind::Singleton, k2.kind());
        assert_eq!(TypeId::of::<i32>(), k2.type_id());
        assert_eq!(
            2500_i32,
            *p2.get_singleton::<i32>().unwrap().lock().unwrap()
        );
    }
}
