use anyhow::Result;
use index::{HashIndex, KeyPosition};
use storage::Segment;
use std::collections::VecDeque;
use std::path::PathBuf;

pub struct KvEngine {
    segments: VecDeque<Segment>,
    index: HashIndex,
    current_segment_size: u64,
    max_segment_size: u64,
    data_dir: PathBuf,
    next_segment_id: u32,
    index_path: PathBuf, 
}

impl KvEngine {
    pub fn new(data_dir: PathBuf, max_segment_size: u64) -> Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        
        let index_path = data_dir.join("index.json");
        
        // Load existing index or create new one
        let index = HashIndex::load_from_file(&index_path).unwrap_or_else(|_| HashIndex::new());
        
        let mut segments = VecDeque::new();
        let segment_path = data_dir.join("segment_0000.db");
        let segment = Segment::new(segment_path)?;
        segments.push_back(segment);
        
        // If index was loaded, we need to find the latest segment
        let mut next_segment_id = 1;
        while data_dir.join(format!("segment_{:04}.db", next_segment_id)).exists() {
            let segment_path = data_dir.join(format!("segment_{:04}.db", next_segment_id));
            let segment = Segment::new(segment_path)?;
            segments.push_back(segment);
            next_segment_id += 1;
        }
        
        Ok(Self {
            segments,
            index,
            current_segment_size: 0,
            max_segment_size,
            data_dir,
            next_segment_id,
            index_path, 
        })
    }
    
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        // Check if we need to rotate segment
        let record_size = (key.len() + value.len() + 2) as u64; // +2 for ':' and '\n'
        if self.current_segment_size + record_size > self.max_segment_size {
            self.rotate_segment()?;
        }
        
        let current_segment = self.segments.back_mut().unwrap();
        let offset = current_segment.append(key, value)?;
        
        // Update index with the new position
        let position = KeyPosition {
            segment_id: self.next_segment_id - 1, // Current segment ID
            offset,
        };
        self.index.set(key.to_string(), position);
        
        // Save index to disk after every write
        self.index.save_to_file(&self.index_path)?;
        
        self.current_segment_size += record_size;
        Ok(())
    }
    
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        if let Some(position) = self.index.get(key) {
            if let Some(segment) = self.segments.get(position.segment_id as usize) {
                let (found_key, value) = segment.read_at(position.offset)?;
                if found_key == key && value != "__TOMBSTONE__" {
                    return Ok(Some(value));
                }
            }
        }
        Ok(None)
    }
    
    fn rotate_segment(&mut self) -> Result<()> {
        let segment_path = self.data_dir.join(format!("segment_{:04}.db", self.next_segment_id));
        let new_segment = Segment::new(segment_path)?;
        
        self.segments.push_back(new_segment);
        self.current_segment_size = 0;
        self.next_segment_id += 1;
        
        Ok(())
    }
}