#![deny(missing_docs)]
//! A simple key/value store.

use std::collections::HashMap;

/// `KvStore` core of database, store key value pair
/// 
/// Example:
/// ```rust 
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key1".to_string(),"value1".to_string());
/// let v = store.get("key1".to_string());
/// assert_eq!(v, Some("value1".to_string()));
/// ```
pub struct KvStore {
    c: HashMap<String, String>,
}

impl KvStore {
    /// create a new KvStore instance
    pub fn new() -> KvStore {
        KvStore { c: HashMap::new() }
    }
    /// set one key value to store
    pub fn set(&mut self, key: String, value: String) {
        self.c.insert(key, value);
    }

    /// get String value by key  
    /// if cannot find key return `None `
    pub fn get(&self, key: String) -> Option<String> {
        self.c.get(&key).cloned()
    }

    /// remove key in store
    pub fn remove(&mut self, key: String) {
        self.c.remove(&key);
    }
}
