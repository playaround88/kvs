use crate::Result;

mod kvs;
mod sled;

pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;

/// plugable engine interface
pub trait KvsEngine {
    /// set key to value
    fn set(&mut self, key: String, value: String) -> Result<()>;
    /// get value of key
    fn get(&mut self, key: String) -> Result<Option<String>>;
    /// remove key in store
    fn remove(&mut self, key: String) -> Result<()>;
}
