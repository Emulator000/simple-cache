use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use async_std::sync::RwLock;

use arc_swap::ArcSwap;

type AnyObject = Box<dyn Any + Send + Sync>;
type CacheObject = Option<ArcSwap<AnyObject>>;

pub trait CacheItem: Send + Sync {}

pub enum CacheResult {
    Ok,
    Error,
}

#[derive(Clone)]
pub struct Cache<K> {
    items: Arc<RwLock<HashMap<K, CacheObject>>>,
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

    pub async fn get<T: 'static + CacheItem, Q: ?Sized>(&self, key: &Q) -> Option<Option<Arc<T>>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.items.read().await.get(key) {
            Some(object) => match object {
                Some(object) => Some(match object.load().downcast_ref::<Arc<T>>() {
                    Some(value) => Some(value.to_owned()),
                    None => None,
                }),
                None => Some(None),
            },
            None => None,
        }
    }

    pub async fn insert<T: 'static + CacheItem>(&self, key: K, value: Option<T>) -> CacheResult {
        match self.items.write().await.insert(
            key,
            match value {
                Some(value) => Some(ArcSwap::new(Arc::new(
                    Box::new(Arc::new(value)) as AnyObject
                ))),
                None => None,
            },
        ) {
            Some(_) => CacheResult::Ok,
            None => CacheResult::Error,
        }
    }

    pub async fn remove<Q: ?Sized>(&self, key: &Q) -> CacheResult
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.items.write().await.remove(key) {
            Some(_) => CacheResult::Ok,
            None => CacheResult::Error,
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

#[cfg(test)]
mod tests {
    use crate::{Cache, CacheItem};

    struct Object {
        value: i32,
        string: String,
    }

    impl CacheItem for Object {}

    #[tokio::test]
    async fn insert_and_get() {
        let cache = Cache::new();
        let object = Object {
            value: 1,
            string: String::from("test!"),
        };

        cache.insert("test", Some(object)).await;

        let cached_object = cache.get::<Object, _>("test").await.unwrap().unwrap();

        assert_eq!(cached_object.value, 1);
        assert_eq!(cached_object.string, "test!");
    }

    #[tokio::test]
    async fn remove() {
        let cache = Cache::new();
        let object = Object {
            value: 1,
            string: String::from("test!"),
        };

        cache.insert("test", Some(object)).await;
        cache.remove("test").await;

        assert_eq!(cache.get::<Object, _>("test").await.is_none(), true);
    }
}
