use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KeyPosition {
    pub segment_id: u32,
    pub offset: u64,
}

pub struct HashIndex {
    index: HashMap<String, KeyPosition>,
}

impl HashIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: String, position: KeyPosition) {
        self.index.insert(key, position);
    }
    
    pub fn get(&self, key: &str) -> Option<&KeyPosition> {
        self.index.get(key)
    }
    
    pub fn remove(&mut self, key: &str) -> Option<KeyPosition> {
        self.index.remove(key)
    }
    
    pub fn contains_key(&self, key: &str) -> bool {
        self.index.contains_key(key)
    }
}