use thiserror::Error;
#[derive(Debug, Error)]
pub enum StorageError {}

pub type StorageResult<T> = Result<T, StorageError>;

pub trait Storage<K, V> {
    fn get(&self, key: &K) -> StorageResult<Option<V>>;
    fn set(&self, key: K, value: V) -> StorageResult<()>;
}
