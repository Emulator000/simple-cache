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

    let cache_result = cache.insert("test", Some(object));

    assert!(cache_result.is_ok());
    assert!(cache_result.unwrap().is_none());

    let cached_object = cache.get::<Object, _>("test").unwrap().unwrap();

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

    let _ = cache.insert("test", Some(object));
    let _ = cache.remove("test");

    let cache_result = cache.get::<Object, _>("test");

    assert!(cache_result.is_ok());
    assert!(cache_result.unwrap().is_none());
}
