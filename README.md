# KV Database

A simple, fast, and persistent key-value database built in Rust. This project implements an append-only storage engine with an in-memory hash index for efficient lookups.

##  Features

- **Append-Only Storage**: Never modifies existing data, only appends new records
- **In-Memory Index**: O(1) lookup time using a hash map
- **Segment Rotation**: Automatically creates new segment files to prevent unlimited growth
- **Crash Safety**: Write-ahead logging and immutable data files
- **Simple CLI**: Easy-to-use command-line interface

