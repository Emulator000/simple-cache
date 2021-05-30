use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use arc_swap::ArcSwap;

type AnyObject = Box<dyn Any + Send + Sync>;
type CacheObject = ArcSwap<AnyObject>;
type CacheResult<T> = Result<T, CacheError>;

pub trait CacheItem: Send + Sync {}

#[derive(Debug)]
pub enum CacheError {
    ReadError,
    WriteError,
    NotFound,
    ValueMismatch,
}

#[derive(Clone)]
pub struct Cache<K> {
    items: Arc<RwLock<HashMap<K, Option<CacheObject>>>>,
}

impl<K> Cache<K>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get<T: 'static + CacheItem, Q: ?Sized>(
        &self,
        key: &Q,
    ) -> CacheResult<Option<Option<Arc<T>>>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.items.read() {
            Ok(map) => match map.get(key) {
                Some(object) => match Self::downcast_object(object) {
                    Ok(value) => Ok(Some(value)),
                    Err(e) => Err(e),
                },
                None => Ok(None),
            },
            Err(_) => Err(CacheError::ReadError),
        }
    }

    pub fn insert<T: 'static + CacheItem>(
        &self,
        key: K,
        value: Option<T>,
    ) -> CacheResult<Option<Arc<T>>> {
        match self.items.write() {
            Ok(mut map) => {
                match map.insert(
                    key,
                    value.map(|value| {
                        ArcSwap::new(Arc::new(Box::new(Arc::new(value)) as AnyObject))
                    }),
                ) {
                    Some(object) => Self::downcast_object(&object),
                    None => Ok(None),
                }
            }
            Err(_) => Err(CacheError::WriteError),
        }
    }

    pub fn remove<Q: ?Sized>(&self, key: &Q) -> CacheResult<Option<CacheObject>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.items.write() {
            Ok(mut map) => match map.remove(key) {
                Some(value) => Ok(value),
                None => Err(CacheError::NotFound),
            },
            Err(_) => Err(CacheError::WriteError),
        }
    }

    fn downcast_object<T: 'static + CacheItem>(
        object: &Option<CacheObject>,
    ) -> CacheResult<Option<Arc<T>>> {
        match object {
            Some(object) => match object.load().downcast_ref::<Arc<T>>() {
                Some(value) => Ok(Some(value.to_owned())),
                None => Err(CacheError::ValueMismatch),
            },
            None => Ok(None),
        }
    }
}

impl<K> Default for Cache<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Cache::new()
    }
}
