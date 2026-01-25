# storage-core

storage-core provides a lightweight repository pattern implementation backed by binary files. Each collection is stored as a single append-only binary file with an in-memory index for fast lookups.

## Features

- **Simple CRUD operations** - Insert, find, update, delete
- **Collection-based** - Organize data into named collections
- **Append-only log** - Write-optimized with crash safety
- **Fast lookups** - In-memory offset map for O(1) retrieval by ID
- **Forward-compatible format** - Versioned binary headers for future extensions
- **Async-ready** - Trait supports both sync and async implementations

## The Repository Trait

```rust
#[async_trait]
pub trait Repository: Send {
    async fn insert(&mut self, repo: M) -> Result; 
    async fn delete(&mut self, id: K) -> Result; 
    async fn find_by_id(&mut self, id: K) -> Option;
    async fn find_all(&mut self) -> Vec;
    async fn update(&mut self, repo: M) -> Result; 
}
```

## File Format

Each collection is stored as a single `.bin` file with the following structure:

```
[RecordHeader: 32 bytes]  ← Version, type, length, timestamp, CRC32, flags
[BSON payload]            ← Serialized document

[RecordHeader: 32 bytes]
[BSON payload]
...
```

**Header fields:**

- Magic number (file format validation)
- Version (schema evolution)
- Record type (Active/Deleted)
- Length (total record size)
- Timestamp (write time)
- CRC32 (corruption detection)
- Flags (compression, encryption, etc.)

## Design Decisions

**Append-only log:**

- Writes always go to end of file
- Updates create new version (old data remains)
- Deletes write tombstone records
- Simple, crash-safe, no corruption risk

**In-memory offset map:**

- Built on startup by scanning file
- Maps ID → file offset
- O(1) lookups by ID
- Trade-off: startup time vs runtime speed

**BSON encoding:**

- Self-describing format
- Handles complex nested data
- Compatible with MongoDB
- Slightly larger than custom binary

**Binary headers:**

- Forward-compatible (version + flags)
- Corruption detection (CRC32)
- Metadata without parsing payload
- Fixed 32-byte size

## Limitations

- No built-in transactions
- No query DSL (use find_all + filter)
- No connection pooling
- File-based implementation is best for small-medium datasets
- **No Transactions** - Operations are not atomic across multiple calls

**Best For:**

- ✅ Applications needing flexible storage abstraction
- ✅ Prototyping and development
- ✅ Small to medium datasets
- ✅ Learning Rust async patterns

**Not Suitable For:**

- ❌ Complex queries or joins
- ❌ Very large datasets
- ❌ ACID transaction requirements

### Basic Example

examples/repo.rs - Simple single-threaded applications
cargo run --example repo

examples/database.rs - Multiple repositories, but accessed one at a time
cargo run --example database

examples/concurrentrs - Web servers, concurrent applications, multiple threads/tasks
cargo run --example concurrent

## Future Work

- [ ] Compaction (remove old versions and tombstones)
- [ ] Persistent offset map (faster startup)
- [ ] Vector index for embeddings (RAG support)
- [ ] Query capabilities (filtering, sorting)
- [ ] Additional backends (MongoDB, PostgreSQL)
- [ ] Transactions
- [ ] Compression

## Contributing

Contributions welcome! Please open an issue or PR.

## License

MIT OR Apache-2.0
