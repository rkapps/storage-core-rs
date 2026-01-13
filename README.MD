# storage-core

A simple, file-based JSON database library for Rust with a generic repository pattern.

## Features

- **Generic Repository Trait** - Type-safe CRUD operations for any serializable type
- **File-per-Record Storage** - Each entity stored as a separate JSON file
- **Async Support** - Built on tokio for non-blocking I/O
- **Easy to Use** - Minimal setup, no external database required
- **Flexible** - No forced concurrency primitives - implementations choose their approach
- **Type Safe** - Leverages Rust's type system and serde

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
storage-core = "0.1.0"  # or path = "../storage-core" for local
```

## Usage

### Basic Example

```rust
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use storage_core::{core::{RepoModel, Repository}, fs::repository::FsRepository};



#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: String,
    name: String,
}

impl RepoModel<String> for User {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn collection(&self) -> &'static str {
        "user"
    }
}


#[tokio::main]
async fn main() -> Result<()>{

    let pb = PathBuf::from("data/tests/users");
    let mut repo = FsRepository::<String, User>::new("users".to_string(), pb);

    // Insert
    let user = User {
        id: "5".to_string(),
        name: "Alice".to_string(),
    };
    
    let _ = repo.insert(user).await?;

    // Find by ID
    let found = repo.find_by_id("5".to_string()).await;
    println!("{:?}", found);

    Ok(())
}
```

## API

### Repository Trait

```rust
#[async_trait]
pub trait Repository<K, M> where M: Send{
   
    async fn insert(&mut self, repo: M) -> Result<()>; 
    async fn delete(&mut self, id: K) -> Result<()>; 
    async fn find_by_id(&mut self, id: K) -> Option<M>;
    async fn find_all(&mut self) -> Vec<M>;
    async fn update(&mut self, repo: M) -> Result<()>; 
}
```

### FileRepository

Implementation of `Repository` that stores data as JSON in a file.

```rust
impl<K, M> Repository<K, M> for FsRepository<K, M>
where
    K: Eq + Hash + Send + Clone + Debug + Display,
    M: RepoModel<K> + Send + Clone + Serialize + Sized + DeserializeOwned,
(

)
```

## How It Works

1. **Storage Format** - Data is stored as a JSON array in multiple files
2. **Thread Safety** - Uses `Mutex` for concurrent access
3. **Automatic Serialization** - Leverages `serde` for JSON conversion
4. **Simple File I/O** - Reads entire file into memory, modifies, writes back

## Limitations

- **No Indexing** - Linear search for `find_by_id`
- **File Locking** - Basic mutex locking, not suitable for high concurrency
- **No Transactions** - Operations are not atomic across multiple calls

**Use Cases:**

- ✅ Small datasets (< 10,000 records)
- ✅ Development and testing
- ✅ Simple applications without heavy concurrency
- ❌ Production systems with large datasets
- ❌ High-concurrency applications

## Examples

See the `examples/` directory for more usage patterns:

- `repo.rs` - Simple CRUD operations for a repository
- `storage.rs` - Use of the file database with multiple repositories
- `concurrent.rs` - Working with multiple spawned tasks to create users and accounts

## Contributing

Contributions welcome! Please open an issue or PR.

## License

MIT OR Apache-2.0