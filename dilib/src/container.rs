use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::provider::{Provider, ProviderKind};
use crate::scoped::Scoped;
use crate::{Injectable, InjectionKey};

pub type Singleton<T> = Arc<Mutex<T>>;

#[derive(Default, Clone)]
pub struct Container<'a> {
    providers: HashMap<InjectionKey<'a>, Provider>,
}

impl<'a> Container<'a> {
    pub fn new() -> Self {
        Container {
            providers: Default::default(),
        }
    }

    pub fn register_scoped<T, F>(&mut self, f: F)
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        self.register_scoped_with_name(None, f)
    }

    pub fn register_scoped_with_name<T, F>(&mut self, name: Option<&str>, f: F)
    where
        T: 'static,
        F: Fn() -> T + 'static,
    {
        let scoped = Scoped::from_factory(f);
        let name = name.map(|s| s.to_string());
        self.register_provider::<T>(Provider::Scoped(scoped), name);
    }

    pub fn register_singleton<T>(&mut self, value: T)
    where
        T: Send + Sync + 'static,
    {
        self.register_singleton_with_name(None, value);
    }

    pub fn register_singleton_with_name<T>(&mut self, name: Option<&str>, value: T)
    where
        T: Send + Sync + 'static,
    {
        let singleton = Arc::new(Mutex::new(value));
        let name = name.map(|s| s.to_string());
        self.register_provider::<T>(Provider::Singleton(singleton), name);
    }

    pub fn register_deps<T>(&mut self)
    where
        T: Injectable + 'static,
    {
        self.register_deps_with_name::<T>(None);
    }

    pub fn register_deps_with_name<T>(&mut self, name: Option<&str>)
    where
        T: Injectable + 'static,
    {
        let scoped = Scoped::from_injectable(T::resolve);
        let name = name.map(|s| s.to_string());
        self.register_provider::<T>(Provider::Scoped(scoped), name);
    }

    pub fn get_scoped<T>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.get_scoped_internal::<T>(None)
    }

    pub fn get_scoped_with_name<T>(&self, name: &str) -> Option<T>
    where
        T: 'static,
    {
        self.get_scoped_internal::<T>(Some(name))
    }

    pub fn get_singleton<T>(&self) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
    {
        self.get_singleton_internal::<T>(None)
    }

    pub fn get_singleton_with_name<T>(&self, name: &str) -> Option<Singleton<T>>
    where
        T: Send + Sync + 'static,
    {
        self.get_singleton_internal::<T>(Some(name))
    }

    pub fn contains(&self, key: InjectionKey) -> bool {
        self.providers.contains_key(&key)
    }

    pub fn remove(&mut self, key: InjectionKey<'a>) -> Option<Provider> {
        self.providers.remove(&key)
    }

    pub fn len(&self) -> usize {
        self.providers.len()
    }

    pub fn clear(&mut self) {
        self.providers.clear();
    }

    ////// Private methods

    fn get_provider(&self, key: InjectionKey<'a>) -> Option<&Provider> {
        self.providers.get(&key)
    }

    fn register_provider<T: 'static>(&mut self, provider: Provider, name: Option<String>) {
        let kind = provider.kind();
        let type_id = TypeId::of::<T>();
        let key = InjectionKey::new(type_id, kind, name);
        self.providers.insert(key, provider);
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

        match self.get_provider(key)? {
            Provider::Scoped(_) => unreachable!(),
            Provider::Singleton(value) => {
                let singleton = value.clone().downcast().ok()?;
                return Some(singleton);
            }
        }
    }
}