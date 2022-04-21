use crate::provider::Provider;
use crate::scoped::Scoped;
use crate::{Inject, InjectionKey, Resolved, Shared};
use std::any::TypeId;
use std::collections::hash_map::{Iter, Values};
use std::collections::HashMap;
use std::sync::Arc;

/// A convenient singleton type.
pub type Singleton<T> = Arc<T>;

/// Represents a store to register and retrieve objects.
#[derive(Default, Clone)]
pub struct Container<'a> {
    providers: HashMap<InjectionKey<'a>, Provider<'a>>,
}

impl<'a> Container<'a> {
    /// Constructs a new `Container`.
    pub fn new() -> Self {
        Container {
            providers: Default::default(),
        }
    }

    /// Adds a scoped factory function.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_scoped<T, F>(&mut self, f: F) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        self.add_scoped_internal::<T>(Scoped::from_factory(f), None)
    }

    /// Adds a scoped factory function with a name.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_scoped_with_name<T, F>(&mut self, name: &str, f: F) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        self.add_scoped_internal::<T>(Scoped::from_factory(f), Some(name))
    }

    /// Adds a singleton.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_singleton<T>(&mut self, value: T) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
    {
        self.add_singleton_internal::<T>(None, Shared::new(value))
    }

    /// Adds a singleton with a name.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_singleton_with_name<T>(&mut self, name: &str, value: T) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
    {
        self.add_singleton_internal::<T>(Some(name), Shared::new(value))
    }

    #[inline]
    #[cfg(feature = "lazy")]
    pub fn add_lazy_singleton<T, F>(&mut self, f: F) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
        F: FnOnce(&Container) -> T + Send + Sync + 'static,
    {
        self.add_singleton_internal::<T>(None, Shared::new_lazy(f))
    }

    #[inline]
    #[cfg(feature = "lazy")]
    pub fn add_lazy_singleton_with_name<T, F>(&mut self, name: &str, f: F) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
        F: FnOnce(&Container) -> T + Send + Sync + 'static,
    {
        self.add_singleton_internal::<T>(Some(name), Shared::new_lazy(f))
    }

    /// Adds a scoped `Inject` that depends on others providers.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_deps<T>(&mut self) -> Result<(), Provider>
    where
        T: Inject + Send + Sync + 'static,
    {
        self.add_scoped_internal::<T>(Scoped::from_construct(T::inject), None)
    }

    /// Adds a scoped named `Inject` that depends on others providers.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_deps_with_name<T>(&mut self, name: &str) -> Result<(), Provider>
    where
        T: Inject + Send + Sync + 'static,
    {
        self.add_scoped_internal::<T>(Scoped::from_construct(T::inject), Some(name))
    }

    /// Adds a scoped `Inject` that depends on others providers.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_deps_fn<T, F>(&mut self, f: F) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        self.add_scoped_internal::<T>(Scoped::from_construct(f), None)
    }

    /// Adds a scoped named `Inject` that depends on others providers.
    ///
    /// # Returns
    /// `Ok(())` if the provider was added, or `Err(Provider)` if
    /// there is a provider registered for that type.
    #[inline]
    pub fn add_deps_fn_with_name<T, F>(&mut self, name: &str, f: F) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        self.add_scoped_internal::<T>(Scoped::from_construct(f), Some(name))
    }

    /// Returns a value registered for the given type or `None`
    /// if no provider is register for the given type.
    ///
    /// The returning value could be either scoped or a singleton.
    #[inline]
    pub fn get<T>(&self) -> Option<Resolved<T>>
    where
        T: Send + Sync + 'static,
    {
        self.get_internal::<T>(None)
    }

    /// Returns a value registered for the given type and name or `None`
    /// if no provider is register for the given type.
    ///
    /// The returning value could be either scoped or a singleton.
    #[inline]
    pub fn get_with_name<T>(&self, name: &str) -> Option<Resolved<T>>
    where
        T: Send + Sync + 'static,
    {
        self.get_internal::<T>(Some(name))
    }

    /// Returns all the values registered for the given type.
    pub fn get_all<T>(&self) -> Vec<Resolved<T>>
    where
        T: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        self.iter()
            .filter(|(key, _)| key.type_id() == type_id)
            .map(|(_, provider)| self.get_resolver_for(provider))
            .flatten()
            .collect()
    }

    /// Returns a value registered for the given type, or `None`
    /// if no provider is register for the given type.
    #[inline]
    pub fn get_scoped<T>(&self) -> Option<T>
    where
        T: Send + Sync + 'static,
    {
        self.get_internal::<T>(None).and_then(|r| r.into_scoped())
    }

    /// Returns a value registered for the given type and name, or `None`
    /// if no provider is register for the given type and name.
    #[inline]
    pub fn get_scoped_with_name<T>(&self, name: &str) -> Option<T>
    where
        T: Send + Sync + 'static,
    {
        self.get_internal::<T>(Some(name))
            .and_then(|r| r.into_scoped())
    }

    /// Returns a singleton registered for the given type, or `None`
    /// if no provider is register for the given type.
    #[inline]
    pub fn get_singleton<T>(&self) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
    {
        self.get_internal::<T>(None)
            .and_then(|r| r.into_singleton())
    }

    /// Returns a singleton registered for the given type and name, or `None`
    /// if no provider is register for the given type and name.
    #[inline]
    pub fn get_singleton_with_name<T>(&self, name: &str) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
    {
        self.get_internal::<T>(Some(name))
            .and_then(|r| r.into_singleton())
    }

    /// Returns `true` if the `Container` have a provider for the given `InjectionKey`.
    #[inline]
    pub fn contains(&self, key: InjectionKey) -> bool {
        self.providers.contains_key(&key)
    }

    /// Removes the provider with the given `InjectionKey` and returns it,
    /// or `None` if the provider is not found.
    #[inline]
    pub fn remove(&mut self, key: InjectionKey<'a>) -> Option<Provider> {
        self.providers.remove(&key)
    }

    /// Returns the number of providers in this `Container`.
    #[inline]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns `true` is this container have no providers.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Removes all the providers in this `Container`.
    #[inline]
    pub fn clear(&mut self) {
        self.providers.clear();
    }

    /// Returns an iterator over the providers of this container.
    #[inline]
    pub fn providers(&self) -> Values<'_, InjectionKey<'a>, Provider> {
        self.providers.values()
    }

    /// Returns an iterator over the keys and providers of this container.
    #[inline]
    pub fn iter(&self) -> Iter<'_, InjectionKey<'a>, Provider> {
        self.providers.iter()
    }

    ////// Helper methods

    fn add_scoped_internal<T>(&mut self, scoped: Scoped, name: Option<&str>) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
    {
        let name = name.map(|s| s.to_string());
        self.add_provider::<T>(Provider::Scoped(scoped), name)
    }

    fn add_singleton_internal<T>(
        &mut self,
        name: Option<&str>,
        shared: Shared<'a>,
    ) -> Result<(), Provider>
    where
        T: Send + Sync + 'static,
    {
        let name = name.map(|s| s.to_string());
        self.add_provider::<T>(Provider::Singleton(shared), name)
    }

    fn get_internal<T>(&self, name: Option<&str>) -> Option<Resolved<T>>
    where
        T: Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, name);

        if let Some(provider) = self.providers.get(&key) {
            self.get_resolver_for(provider)
        } else {
            None
        }
    }

    pub fn get_resolver_for<T>(&self, provider: &Provider) -> Option<Resolved<T>>
    where
        T: Send + Sync + 'static,
    {
        match provider {
            Provider::Scoped(x) => match x {
                Scoped::Factory(_) => x.call_factory().map(|x| Resolved::Scoped(x)),
                Scoped::Construct(_) => x.call_construct(self).map(|x| Resolved::Scoped(x)),
            },
            Provider::Singleton(x) => match x {
                Shared::Instance(_) => x.get().map(Resolved::Singleton),

                #[cfg(feature = "lazy")]
                Shared::Lazy(_) => x.get_with(self).map(Resolved::Singleton),

                #[cfg(not(feature = "lazy"))]
                Shared::__NonExhaustive(_) => None,
            },
        }
    }

    pub(crate) fn add_provider<T: 'static>(
        &mut self,
        provider: Provider<'a>,
        name: Option<String>,
    ) -> Result<(), Provider> {
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, name);
        self.add_provider_internal(key, provider)
    }

    pub(crate) fn add_provider_internal(
        &mut self,
        key: InjectionKey<'a>,
        provider: Provider<'a>,
    ) -> Result<(), Provider> {
        match self.providers.insert(key, provider) {
            Some(x) => Err(x),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn scoped_test() {
        let mut container = Container::new();
        container.add_scoped(|| "hello world").unwrap(); // &str

        assert_eq!(container.len(), 1);

        let value = container.get_scoped::<&str>().unwrap();
        assert_eq!(value, "hello world");

        assert!(container.get_scoped::<String>().is_none());
    }

    #[test]
    fn scoped_with_name_test() {
        let mut container = Container::new();
        container
            .add_scoped_with_name("greet", || "hello world")
            .unwrap(); // &str

        assert_eq!(container.len(), 1);

        let value = container.get_scoped_with_name::<&str>("greet").unwrap();
        assert_eq!(value, "hello world");

        assert!(container.get_scoped_with_name::<String>("greet").is_none());
        assert!(container.get_scoped_with_name::<&str>("saludo").is_none());
    }

    #[test]
    fn singleton_test() {
        let mut container = Container::new();
        container.add_singleton(42069_i32).unwrap();

        assert_eq!(container.len(), 1);

        let value = container.get_singleton::<i32>().unwrap();

        assert_eq!(*value, 42069_i32);
        assert!(container.get_singleton::<i64>().is_none());
    }

    #[test]
    fn singleton_with_name_test() {
        let mut container = Container::new();
        container
            .add_singleton_with_name("funny number", 42069_i32)
            .unwrap();

        assert_eq!(container.len(), 1);

        let value = container
            .get_singleton_with_name::<i32>("funny number")
            .unwrap();

        assert_eq!(*value, 42069_i32);
        assert!(container.get_singleton_with_name::<i32>("number").is_none());
    }

    #[test]
    #[cfg(feature = "lazy")]
    fn lazy_singleton_test() {
        let mut container = Container::new();
        container
            .add_lazy_singleton(|_| Mutex::new(128_isize))
            .unwrap();

        let s1 = container.get_singleton::<Mutex<isize>>().unwrap();
        assert_eq!(*s1.lock().unwrap(), 128_isize);

        {
            *s1.lock().unwrap() += 128;
        }

        let s2 = container.get_singleton::<Mutex<isize>>().unwrap();
        assert_eq!(*s2.lock().unwrap(), 256_isize);
    }

    #[test]
    #[cfg(feature = "lazy")]
    fn lazy_singleton_with_name_test() {
        let mut container = Container::new();
        container
            .add_lazy_singleton_with_name("bits", |_| Mutex::new(128_isize))
            .unwrap();

        let s1 = container
            .get_singleton_with_name::<Mutex<isize>>("bits")
            .unwrap();
        assert_eq!(*s1.lock().unwrap(), 128_isize);

        {
            *s1.lock().unwrap() += 128;
        }

        let s2 = container
            .get_singleton_with_name::<Mutex<isize>>("bits")
            .unwrap();
        assert_eq!(*s2.lock().unwrap(), 256_isize);
    }

    #[test]
    fn contains_test() {
        let mut container = Container::new();
        container.add_scoped(|| 200_i32).unwrap();
        container
            .add_scoped_with_name("number", || 999_i32)
            .unwrap();
        container
            .add_singleton(String::from("have a good day"))
            .unwrap();
        container
            .add_singleton_with_name("bye", "adios amigo")
            .unwrap();

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

        impl Inject for Greeter {
            fn inject(container: &Container) -> Self {
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
        container
            .add_singleton_with_name("counter", Mutex::new(0_usize))
            .unwrap();
        container.add_deps::<Greeter>().unwrap();
        container
            .add_scoped_with_name("en_msg", || String::from("hello"))
            .unwrap();

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

        impl Inject for Greeter {
            fn inject(container: &Container) -> Self {
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
        container
            .add_singleton_with_name("counter", Mutex::new(0_usize))
            .unwrap();
        container
            .add_deps_with_name::<Greeter>("en_greeter")
            .unwrap();
        container
            .add_scoped_with_name("en_msg", || String::from("hello"))
            .unwrap();

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
    fn remove_test() {
        let mut container = Container::new();
        assert_eq!(container.len(), 0);

        container.add_scoped(|| true).unwrap();
        container.add_singleton(String::from("blue")).unwrap();
        container
            .add_scoped_with_name("number", || 200_i32)
            .unwrap();
        container
            .add_singleton_with_name("color", String::from("red"))
            .unwrap();

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

        container.add_scoped(|| true).unwrap();
        container.add_singleton(String::from("blue")).unwrap();

        assert_eq!(container.len(), 2);

        container.clear();
        assert_eq!(container.len(), 0);
    }

    #[test]
    fn providers_test() {
        let mut container = Container::new();
        container.add_scoped(|| true).unwrap();
        container.add_singleton(0.25_f32).unwrap();
        container.add_scoped(|| 200_usize).unwrap();

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
        container
            .add_scoped_with_name("truthfulness", || true)
            .unwrap();
        container.add_singleton(2500_i32).unwrap();

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

    #[test]
    fn get_test() {
        let mut container = Container::new();
        container.add_scoped(|| String::from("orange")).unwrap();
        container.add_singleton(12_usize).unwrap();

        assert_eq!(
            Some(&String::from("orange")),
            container.get::<String>().as_deref()
        );
        assert_eq!(Some(&12_usize), container.get::<usize>().as_deref());
    }

    #[test]
    fn get_with_name_test() {
        let mut container = Container::new();
        container.add_scoped_with_name("truthy", || true).unwrap();
        container.add_singleton_with_name("falsy", false).unwrap();

        assert_eq!(
            Some(&true),
            container.get_with_name::<bool>("truthy").as_deref()
        );

        assert_eq!(
            Some(&false),
            container.get_with_name::<bool>("falsy").as_deref()
        );
    }

    #[test]
    fn get_all_test() {
        let mut container = Container::new();
        container.add_scoped(|| 69_i32).unwrap();
        container.add_scoped_with_name("truthy", || 42_i32).unwrap();
        container.add_singleton_with_name("funny_number", 420_i32).unwrap();

        let values = container.get_all::<i32>();
        assert_eq!(3, values.len());

        let x1 = values.iter().find(|v| *v.as_ref() == 69_i32).unwrap();
        let x2 = values.iter().find(|v| *v.as_ref() == 42_i32).unwrap();
        let x3 = values.iter().find(|v| *v.as_ref() == 420_i32).unwrap();

        assert_eq!(69_i32, x1.cloned());
        assert_eq!(42_i32, x2.cloned());
        assert_eq!(420_i32, x3.cloned());
    }
}
