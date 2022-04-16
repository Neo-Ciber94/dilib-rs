use crate::repositories::Entity;
use crate::Repository;
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::ErrorKind;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::Error;
use tokio::sync::RwLock;

type DataMap = HashMap<String, HashMap<String, String>>;

static IS_INIT: AtomicBool = AtomicBool::new(false);
static PHYSICAL_STORAGE: OnceCell<RwLock<PhysicalStorage>> = OnceCell::new();

async fn get_physical_storage() -> &'static RwLock<PhysicalStorage> {
    if !IS_INIT.load(Ordering::SeqCst) {
        let physical_storage = PhysicalStorage::load("data/data.json")
            .await
            .expect("Failed to load data");

        let ret = PHYSICAL_STORAGE.get_or_init(|| RwLock::new(physical_storage));
        IS_INIT.store(true, Ordering::SeqCst);
        return ret;
    }

    PHYSICAL_STORAGE.get().unwrap()
}

struct PhysicalStorage {
    inner: DataMap,
    path: PathBuf,
}
impl PhysicalStorage {
    pub async fn load(file_name: &str) -> Result<PhysicalStorage, Error> {
        if file_name.starts_with('/') {
            panic!("file_name must not start with '/'");
        }

        let mut path = std::path::PathBuf::new();
        path.push(std::env::current_dir().expect("current dir"));
        path.push(file_name);

        if !std::path::Path::new(&path).exists() {
            return Ok(PhysicalStorage {
                inner: DataMap::new(),
                path,
            });
        }

        let json = tokio::fs::read_to_string(&path).await?;
        match serde_json::from_str::<DataMap>(&json) {
            Ok(data) => Ok(PhysicalStorage { inner: data, path }),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    pub async fn save(&self) -> Result<(), Error> {
        if !self.path.exists() {
            tokio::fs::create_dir_all(self.path.parent().unwrap()).await?;
        }

        let json = serde_json::to_string(&self.inner).unwrap();
        tokio::fs::write(&self.path, json).await?;
        Ok(())
    }

    pub async fn get_all<K, V>(&self, key: &str) -> Result<HashMap<K, V>, Error>
    where
        K: Hash + Eq + Serialize + DeserializeOwned + 'static,
        V: Serialize + DeserializeOwned + 'static,
    {
        if let Some(map) = self.inner.get(key) {
            let mut result = HashMap::new();
            for (id, json) in map {
                let key = serde_json::from_str::<K>(&id).expect("Failed to parse key");
                let value = serde_json::from_str::<V>(&json).expect("Failed to parse value");
                result.insert(key, value);
            }
            Ok(result)
        } else {
            Ok(HashMap::new())
        }
    }

    pub async fn with<K, V, R, F>(&self, key: &str, f: F) -> Result<R, Error>
    where
        F: FnOnce(HashMap<K, V>) -> R,
        K: Hash + Eq + Serialize + DeserializeOwned + 'static,
        V: Serialize + DeserializeOwned + 'static,
    {
        let map = self.get_all(key).await?;
        Ok(f(map))
    }

    pub async fn with_save<K, V, R, F>(&mut self, key: &str, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut HashMap<K, V>) -> R,
        K: Hash + Eq + Serialize + DeserializeOwned + 'static,
        V: Serialize + DeserializeOwned + 'static,
    {
        let mut map = self.get_all(key).await?;
        let result = f(&mut map);

        let to_save = map
            .iter()
            .map(|(k, v)| {
                let key_json = serde_json::to_string(k).unwrap();
                let value_json = serde_json::to_string(v).unwrap();
                (key_json, value_json)
            })
            .collect();

        self.inner.insert(key.to_string(), to_save);
        self.save().await?;
        Ok(result)
    }
}

pub struct StorageRepository<T, Id> {
    key: String,
    _marker: PhantomData<(T, Id)>,
}
impl<T, Id> StorageRepository<T, Id> {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            _marker: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T, Id> Repository<T, Id> for StorageRepository<T, Id>
where
    T: Entity<Id> + Sync + Send + Clone + DeserializeOwned + Serialize + 'static,
    Id: Hash + Eq + Sync + Send + Clone + DeserializeOwned + Serialize + 'static,
{
    async fn get_all(&self) -> Vec<T> {
        let storage = get_physical_storage().await.read().await;
        let map = storage.get_all::<Id, T>(&self.key).await.unwrap();
        map.values().cloned().collect()
    }

    async fn get(&self, id: Id) -> Option<T> {
        let storage = get_physical_storage().await.read().await;
        storage
            .with(&self.key, move |map: HashMap<Id, T>| map.get(&id).cloned())
            .await
            .unwrap()
    }

    async fn add(&mut self, entity: T) -> T {
        let mut storage = get_physical_storage().await.write().await;
        storage
            .with_save(&self.key, move |map: &mut HashMap<Id, T>| {
                let id = entity.id().clone();
                match map.insert(id, entity.clone()) {
                    Some(old) => old,
                    None => entity,
                }
            })
            .await
            .unwrap()
    }

    async fn update(&mut self, entity: T) -> Option<T> {
        let mut storage = get_physical_storage().await.write().await;
        storage
            .with_save(&self.key, move |map: &mut HashMap<Id, T>| {
                let id = entity.id().clone();
                match map.entry(id) {
                    Entry::Occupied(mut entry) => {
                        let old_entity = entry.get_mut();
                        *old_entity = entity;
                        Some(old_entity.clone())
                    }
                    Entry::Vacant(_) => None,
                }
            })
            .await
            .unwrap()
    }

    async fn delete(&mut self, id: Id) -> Option<T> {
        let mut storage = get_physical_storage().await.write().await;
        storage
            .with_save(&self.key, move |map: &mut HashMap<Id, T>| map.remove(&id))
            .await
            .unwrap()
    }
}
