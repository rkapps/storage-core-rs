use std::{
    any::Any,
    fmt::{Debug, Display},
    hash::Hash,
};

use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

use crate::fs::search::SearchCriteria;


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

    async fn find(&mut self, search: Option<SearchCriteria>) -> Vec<M>
        where M: Searchable;

    async fn semantic_search(
        &mut self,
        query_vector: &[f32],
        top_k: usize,
        criteria: Option<SearchCriteria>,
    ) -> Vec<(M, f32)>
    where
        M: VectorEmbedding + Searchable + RepoModel<K>;
}

#[async_trait]
pub trait Initializable: Send + Sync + Debug {
    async fn initialize(&mut self) -> Result<()>;
    async fn shutdown(&mut self) -> Result<()>;
    fn as_any(&mut self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// VectorEmbedding trait - needs to implement id and vectors
pub trait VectorEmbedding: Send + Sync + Debug {
    fn vector(&self) -> &[f32];
}

//Searchable trait 
pub trait Searchable {
    fn matches_filter(&self, _criteria: &SearchCriteria) -> bool {
        true  // Default: pass all (no Searching)
    }

    fn get_field_value(&self, field: &str) -> Option<SortValue>; 

}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum SortValue {
    String(String),
    Decimal(rust_decimal::Decimal),
    Int(i64),
}