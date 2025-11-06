use anyhow::Result;
use clap::Parser;
use cli::Cli;
use engine::KvEngine;
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize database engine
    let data_dir = PathBuf::from("./data");
    std::fs::create_dir_all(&data_dir)?;
    
    let mut engine = KvEngine::new(data_dir, 1024 * 1024)?; // 1MB segments
    
    match cli.command {
        cli::Commands::Get { key } => {
            if let Some(value) = engine.get(&key)? {
                println!("{}", value);
            } else {
                println!("Key not found: {}", key);
            }
        }
        cli::Commands::Set { key, value } => {
            engine.set(&key, &value)?;
            println!("OK");
        }
        cli::Commands::Delete { key } => {
            engine.set(&key, "__TOMBSTONE__")?;
            println!("OK");
        }
    }
    
    Ok(())
}