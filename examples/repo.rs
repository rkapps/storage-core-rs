pub mod common;
use anyhow::Result;
use std::path::PathBuf;
use storage_core::{
    core::{Initializable, Repository},
    fs::repository::FsRepository,
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::common::models::User;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("storage_core::examples", Level::DEBUG)
        .with_target("storage_core::fs", Level::DEBUG);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty()) // Compact format
        .with(filter)
        .init();

    let pb = PathBuf::from("data/tests/users");
    let mut repo = FsRepository::<String, User>::new("users".to_string(), pb)?;
    repo.initialize().await?;
    // Insert
    let user = User {
        id: "5".to_string(),
        name: "Alice".to_string(),
    };

    let _ = repo.insert(user).await?;

    // Find by ID
    let found = repo.find_by_id("5".to_string()).await;
    println!("Found: {:?}", found);

    Ok(())
}
