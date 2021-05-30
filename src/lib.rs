use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use async_std::sync::RwLock;

use arc_swap::ArcSwap;

type AnyObject = Box<dyn Any + Send + Sync>;
type CacheObject = ArcSwap<AnyObject>;
type CacheResult<T> = Result<T, CacheError>;
type CacheType<T> = Arc<T>;

pub trait CacheItem: Send + Sync {}

#[derive(Debug)]
pub enum CacheError {
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

    pub async fn get<T: 'static + CacheItem, Q: ?Sized>(
        &self,
        key: &Q,
    ) -> CacheResult<Option<CacheType<T>>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.items.read().await.get(key) {
            Some(object) => Self::downcast_object(object),
            None => Ok(None),
        }
    }

    pub async fn insert<T: 'static + CacheItem>(
        &self,
        key: K,
        value: Option<T>,
    ) -> CacheResult<Option<CacheType<T>>> {
        match self.items.write().await.insert(
            key,
            match value {
                Some(value) => Some(ArcSwap::new(Arc::new(
                    Box::new(Arc::new(value)) as AnyObject
                ))),
                None => None,
            },
        ) {
            Some(object) => Self::downcast_object(&object),
            None => Ok(None),
        }
    }

    pub async fn remove<Q: ?Sized>(&self, key: &Q) -> CacheResult<Option<CacheObject>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.items.write().await.remove(key) {
            Some(value) => Ok(value),
            None => Err(CacheError::NotFound),
        }
    }

    fn downcast_object<T: 'static + CacheItem>(
        object: &Option<CacheObject>,
    ) -> CacheResult<Option<CacheType<T>>> {
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
