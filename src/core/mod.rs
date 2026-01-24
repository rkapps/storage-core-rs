use std::{
    any::Any,
    fmt::{Debug, Display},
    hash::Hash,
};

use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

// 1. Define a trait alias to consolidate constraints
pub trait RepoKey:
    Eq + Hash + Send + Sync + Clone + Debug + Display + Serialize + DeserializeOwned + 'static
{
}
impl<T> RepoKey for T where
    T: Eq + Hash + Send + Sync + Clone + Debug + Display + Serialize + DeserializeOwned + 'static
{
}

pub trait RepoModel<K>:
    Send + Sync + Clone + Serialize + Debug + DeserializeOwned + 'static
{
    // Returns the id of the model;
    fn id(&self) -> K;

    // Returns the name of the collection;
    fn collection(&self) -> &'static str;
}

#[async_trait]
pub trait Repository<K, M>: Send + Sync {
    async fn insert(&mut self, repo: M) -> Result<()>;
    async fn delete(&mut self, repo: M) -> Result<()>;
    async fn find_by_id(&mut self, id: K) -> Option<M>;
    async fn find_all(&mut self) -> Vec<M>;
    async fn update(&mut self, repo: M) -> Result<()>;
}

#[async_trait]
pub trait Initializable: Send + Sync + Debug {
    async fn initialize(&mut self) -> Result<()>;
    async fn shutdown(&mut self) -> Result<()>;
    fn as_any(&mut self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
