pub mod common;
use std::path::PathBuf;
use anyhow::Result;
use storage_core::{core::Repository, fs::repository::FsRepository};

use crate::common::models::User;


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