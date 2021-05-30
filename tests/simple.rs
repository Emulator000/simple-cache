use simple_cache::{Cache, CacheItem};

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

    let cache_get = cache.get::<Object, _>("test");

    assert!(cache_get.is_ok());
    assert!(cache_get.unwrap().is_none());

    let cache_insert = cache.insert("test", Some(object));

    assert!(cache_insert.is_ok());
    assert!(cache_insert.unwrap().is_none());

    let cache_get = cache.get::<Object, _>("test").unwrap().unwrap().unwrap();

    assert_eq!(cache_get.value, 1);
    assert_eq!(cache_get.string, "test!");

    let cache_insert = cache.insert::<Object>("test", None);

    assert!(cache_insert.is_ok());
    assert!(cache_insert.unwrap().is_some());

    let cache_get = cache.get::<Object, _>("test").unwrap();

    assert!(cache_get.is_some());
    assert!(cache_get.unwrap().is_none());
}

#[tokio::test]
async fn remove() {
    let cache = Cache::new();
    let object = Object {
        value: 1,
        string: String::from("test!"),
    };

    let _ = cache.insert("test", Some(object));
    let _ = cache.remove("test");

    let cache_get = cache.get::<Object, _>("test");

    assert!(cache_get.is_ok());
    assert!(cache_get.unwrap().is_none());

    let cache_remove = cache.remove("test");

    assert!(cache_remove.is_err());
}
