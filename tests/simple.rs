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
