# storage-core

A lightweight, async storage library for Rust with a generic repository pattern. Provides the trait definition - implementations and concurrency strategies are up to you.

## Features

- **Generic Repository Trait** - Type-safe CRUD operations for any serializable type
- **Async-first** - Built with `async-trait` for non-blocking I/O
- **Flexible Concurrency** - No enforced locking - choose your own strategy
- **Send-safe** - Trait bound ensures thread-safe usage

## Installation

```toml
[dependencies]
storage-core = "0.1.0"
```

## Usage

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

## Design Philosophy

**What storage-core provides:**

- ✅ Repository trait definition
- ✅ Reference file-based implementation
- ✅ Thread-safe design (`: Send` bound)

**What you provide:**

- Your concurrency strategy (Mutex, RwLock, single-threaded)
- Your storage implementation (if not using FsRepository)
- Your error handling approach

This separation gives you maximum flexibility to choose patterns that fit your application.

## Advantages

- ✅ **Flexible** - Choose your own concurrency model
- ✅ **Async-first** - Non-blocking I/O
- ✅ **Type-safe** - Compile-time guarantees
- ✅ **Simple** - Minimal API surface
- ✅ **Thread-safe** - Send bound enables concurrent usage

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

examples/storage.rs - Multiple repositories, but accessed one at a time
cargo run --example storage

examples/concurrentrs - Web servers, concurrent applications, multiple threads/tasks
cargo run --example concurrent

## Contributing

Contributions welcome! Please open an issue or PR.

## License

MIT OR Apache-2.0
