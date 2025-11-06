use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Segment {
    file_path: PathBuf,
    pub size: u64,
}

impl Segment {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&path)
            .with_context(|| format!("Failed to create segment file: {:?}", path))?;
        
        let size = file.metadata()?.len();
        
        Ok(Segment {
            file_path: path,
            size,
        })
    }
    
    pub fn append(&mut self, key: &str, value: &str) -> Result<u64> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
            
        let record = format!("{}:{}\n", key, value);
        let offset = self.size;
        
        write!(file, "{}", record)?;
        file.flush()?;
        
        self.size += record.len() as u64;
        Ok(offset)
    }
    
    pub fn read_at(&self, offset: u64) -> Result<(String, String)> {
        let file = File::open(&self.file_path)?;
        let mut reader = BufReader::new(file);
        
        reader.seek(SeekFrom::Start(offset))?;
        
        let mut line = String::new();
        reader.read_line(&mut line)?;
        
        let line = line.trim();
        if let Some(sep_pos) = line.find(':') {
            let key = &line[..sep_pos];
            let value = &line[sep_pos + 1..];
            Ok((key.to_string(), value.to_string()))
        } else {
            Err(anyhow::anyhow!("Invalid record format: {}", line))
        }
    }
    
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }
}