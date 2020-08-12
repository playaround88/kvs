use crate::{KvsEngine, Result};

extern crate sled;
use sled::Db;

/// sled store engine
pub struct SledKvsEngine {
    tree: Db,
}

impl SledKvsEngine {
    /// start KvStore in path
    pub fn open() -> Result<SledKvsEngine> {
        let tree = sled::open("mysled.db").expect("open");
        Ok(SledKvsEngine { tree })
    }
}

impl KvsEngine for SledKvsEngine {
    /// set key to value
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.tree.insert(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }
    /// get value of key
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let v = self.tree.get(key).unwrap().unwrap().to_vec();
        Ok(Some(String::from_utf8(v)?))
    }
    /// remove key in store
    fn remove(&mut self, key: String) -> Result<()> {
        self.tree.remove(key)?;
        Ok(())
    }
}
