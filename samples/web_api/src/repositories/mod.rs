use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;

mod memory;
mod storage;

pub use {memory::InMemoryRepository, storage::StorageRepository};

pub trait Entity<Id> {
    fn id(&self) -> &Id;
}

#[async_trait::async_trait]
pub trait Repository<T, Id>
where
    T: Entity<Id> + Send + Sync + Clone + DeserializeOwned + Serialize,
    Id: Hash + Eq + Send + Sync + Clone + DeserializeOwned + Serialize,
{
    async fn get_all(&self) -> Vec<T>;
    async fn get(&self, id: Id) -> Option<T>;
    async fn add(&mut self, entity: T) -> T;
    async fn update(&mut self, entity: T) -> Option<T>;
    async fn delete(&mut self, id: Id) -> Option<T>;
}
