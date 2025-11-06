use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "db")]
#[command(about = "A simple key-value database", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get a value by key
    Get {
        key: String,
    },
    /// Set a key-value pair
    Set {
        key: String,
        value: String,
    },
    /// Delete a key
    Delete {
        key: String,
    },
}