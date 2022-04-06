use crate::repository::Entity;
use crate::Repository;
use lazy_static::lazy_static;
use std::any::{TypeId, Any};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

type AnyMap = HashMap<u64, Box<dyn Any + Send + Sync>>;

lazy_static! {
    static ref STORAGE: Arc<RwLock<HashMap<TypeId, AnyMap>>> = Default::default();
}

pub struct InMemoryRepository<T, Id> {
    _id: PhantomData<Id>,
    _t: PhantomData<T>,
}

impl<T, Id> Default for InMemoryRepository<T, Id> {
    fn default() -> Self {
        InMemoryRepository {
            _id: PhantomData,
            _t: PhantomData,
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
            for entity in entities.values() {
                if let Some(e) = entity.downcast_ref::<T>().cloned() {
                    result.push(e);
                }
            }
        }

        result
    }

    async fn get(&self, id: Id) -> Option<T> {
        let type_id = TypeId::of::<T>();

        if let Some(entities) = STORAGE.read().unwrap().get(&type_id) {
            let hash = hash(&id);
            if let Some(entity) = entities.get(&hash) {
                if let Some(e) = entity.downcast_ref::<T>().cloned() {
                    return Some(e);
                }
            }
        }

        None
    }

    async fn add(&mut self, entity: T) -> T {
        let hash = hash(&entity.id());
        let type_id = TypeId::of::<T>();
        let mut lock =  STORAGE.write().unwrap();

        // We only create new map if it doesn't exist
        let entities = lock.entry(type_id).or_insert_with(HashMap::new);
        entities.insert(hash, Box::new(entity.clone()));
        entity
    }

    async fn update(&mut self, entity: T) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let hash = hash(&entity.id());

        if let Some(entities) = STORAGE.write().unwrap().get_mut(&type_id) {
            if let Some(entity_to_update) = entities.get_mut(&hash) {
                *entity_to_update = Box::new(entity.clone());
                return Some(entity);
            }
        }

        None
    }

    async fn delete(&mut self, id: Id) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let hash = hash(&id);

        if let Some(entities) = STORAGE.write().unwrap().get_mut(&type_id) {
            if let Some(entity_to_delete) = entities.remove(&hash) {
                if let Some(e) = entity_to_delete.downcast_ref::<T>() {
                    return Some(e.clone());
                }
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
