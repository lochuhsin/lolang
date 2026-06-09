use std::collections::HashMap;

use crate::values::GenericValue;

#[derive(Debug, Default)]
pub struct Table {
    pub container: HashMap<String, GenericValue>, // Note: Check the performance of a hash table
}

impl Table {
    pub fn new() -> Table {
        Table {
            container: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: GenericValue) -> bool {
        // if the key doesn't exist, return true, else return false
        self.container.insert(key, value).is_none()
    }

    pub fn get(&self, key: &str) -> Option<&GenericValue> {
        self.container.get(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.container.remove(key).is_some()
    }
}
