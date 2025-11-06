use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPosition {
    pub segment_id: u32,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
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
    
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.index)?;
        Ok(())
    }
    
    pub fn load_from_file(path: &Path) -> Result<Self> {
        if path.exists() {
            let file = OpenOptions::new().read(true).open(path)?;
            let reader = BufReader::new(file);
            let index: HashMap<String, KeyPosition> = serde_json::from_reader(reader)?;
            Ok(Self { index })
        } else {
            Ok(Self::new())
        }
    }
}