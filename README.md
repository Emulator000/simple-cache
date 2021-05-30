# Simple Cache
[![Crates.io](https://img.shields.io/crates/v/simple-cache.svg)](https://crates.io/crates/simple-cache)
[![Build Status](https://travis-ci.com/facile-it/rabbitmq-consumer.svg?branch=master)](https://travis-ci.com/facile-it/rabbitmq-consumer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A basic and simple Rust library async/await ready caching implementation for structures.

## Usage
```rust
use simple_cache::{Cache, CacheItem};

struct Object {
    value: i32,
    string: String,
}

impl CacheItem for Object {}

#[tokio::main]
async fn main() {
    let cache = Cache::new();
    let object = Object {
        value: 1,
        string: String::from("test!"),
    };

    let _ = cache.insert("test", Some(object));

    let cached_object = cache.get::<Object, _>("test").unwrap().unwrap().unwrap();
    
    if cached_object.value == 1 {
        println!("Hi from Simple Cache!");
    }
}
```
