use crate::repositories::Entity;
use crate::Repository;
use lazy_static::lazy_static;
use std::any::{Any, TypeId};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

type ObjectMap = Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>;

lazy_static! {
    static ref STORAGE: ObjectMap = Default::default();
}

pub struct InMemoryRepository<T, Id> {
    _marker: PhantomData<(T, Id)>,
}

impl<T, Id> Default for InMemoryRepository<T, Id> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T, Id> Repository<T, Id> for InMemoryRepository<T, Id>
where
    T: Entity<Id> + Sync + Send + Clone + 'static,
    Id: Hash + Sync + Send,
{
    async fn get_all(&self) -> Vec<T> {
        let mut result = Vec::new();
        let type_id = TypeId::of::<T>();

        if let Some(entities) = STORAGE.read().unwrap().get(&type_id) {
            let map = entities.downcast_ref::<HashMap<u64, T>>().unwrap();
            for entity in map.values() {
                result.push(entity.clone());
            }
        }

        result
    }

    async fn get(&self, id: Id) -> Option<T> {
        let type_id = TypeId::of::<T>();

        if let Some(entities) = STORAGE.read().unwrap().get(&type_id) {
            let map = entities.downcast_ref::<HashMap<u64, T>>().unwrap();
            let hash = hash(&id);
            if let Some(entity) = map.get(&hash) {
                return Some(entity.clone());
            }
        }

        None
    }

    async fn add(&mut self, entity: T) -> T {
        let hash = hash(&entity.id());
        let type_id = TypeId::of::<T>();
        let mut lock = STORAGE.write().unwrap();

        // We only create new map if it doesn't exist
        match lock.entry(type_id) {
            Entry::Occupied(any) => {
                let map = any.into_mut().downcast_mut::<HashMap<u64, T>>().unwrap();
                map.insert(hash, entity.clone());
            }
            Entry::Vacant(any) => {
                let mut map = HashMap::new();
                map.insert(hash, entity.clone());
                any.insert(Box::new(map));
            }
        }

        entity
    }

    async fn update(&mut self, entity: T) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let hash = hash(&entity.id());

        if let Some(entities) = STORAGE.write().unwrap().get_mut(&type_id) {
            let map = entities.downcast_mut::<HashMap<u64, T>>().unwrap();
            if let Some(entity_to_update) = map.get_mut(&hash) {
                *entity_to_update = entity.clone();
                return Some(entity);
            }
        }

        None
    }

    async fn delete(&mut self, id: Id) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let hash = hash(&id);

        if let Some(entities) = STORAGE.write().unwrap().get_mut(&type_id) {
            let map = entities.downcast_mut::<HashMap<u64, T>>().unwrap();
            if let Some(entity_to_delete) = map.remove(&hash) {
                return Some(entity_to_delete.clone());
            }
        }

        None
    }
}

fn hash<H>(id: &H) -> u64
where
    H: Hash,
{
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}
