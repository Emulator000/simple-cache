# Simple Cache
A basic and simple Rust library async/await ready caching implementation for structures.

## Usage
```rust
use simple_cache::Cache;

fn main() {
    let cache = Cache::new();
    let object = Object {
        value: 1,
        string: String::from("test!"),
    };

    cache.insert("test", Some(object)).await;

    let cached_object = cache.get::<Object, _>("test").await.unwrap().unwrap();
    
    if cached_object.value == 1 {
        println!("Hi from Simple Cache!");
    }
}
```
