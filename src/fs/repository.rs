use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use std::cmp::Eq;
use std::marker::PhantomData;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    path::PathBuf,
};
use tokio::fs;

use crate::core::{RepoModel, Repository};
use crate::fs::errors::FsRepositoryError;
use crate::fs::utils;

pub struct FsRepository<K, M>
where
    K: Eq + Hash,
    M: RepoModel<K> + DeserializeOwned,
{
    pub name: String,
    collection_path: PathBuf,
    _phantom1: PhantomData<K>,
    _phantom2: PhantomData<M>,
}

impl<K, M> FsRepository<K, M>
where
    K: Eq + Hash + Send + Clone + Debug + Display,
    M: RepoModel<K> + Send + Clone + DeserializeOwned,
{
    pub fn new(name: String, collection_path: PathBuf) -> Self {
        Self {
            name,
            collection_path,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

#[async_trait]
impl<K, M> Repository<K, M> for FsRepository<K, M>
where
    K: Eq + Hash + Send + Clone + Debug + Display,
    M: RepoModel<K> + Send + Clone + Serialize + Sized + DeserializeOwned,
{

    // insert creates a new json file for the chat id. creates the directory structure too.
    async fn insert(&mut self, model: M) -> Result<()> {

        // Create the directories if they do not exist
        if !self.collection_path.exists() {
            let _ = fs::create_dir_all(&self.collection_path)
                .await
                .with_context(|| FsRepositoryError::DirectoryCreation { path: (self.collection_path.clone())})?;
        }

        let pathbuf= utils::build_json_file_path(&self.collection_path, model.id());
        let json = serde_json::to_string_pretty(&model)?;
        fs::write(&pathbuf, json)
            .await
            .with_context(|| FsRepositoryError::FileCreation {path: pathbuf})?;

        Ok(())
    }

    // delete deletes the json file for the chat id.
    async fn delete(&mut self, id: K) -> Result<()> {
        let pathbuf= utils::build_json_file_path(&self.collection_path, id);
        fs::remove_file(&pathbuf)
            .await
            .with_context(|| FsRepositoryError::FileDeletion {path: pathbuf})?;
        Ok(())
    }

    // find_by_id finds the json file for the id, marshalls that into the object
    async fn find_by_id(&mut self, id: K) -> Option<M> {
        let pathbuf= utils::build_json_file_path(&self.collection_path, id);
        let contents = tokio::fs::read_to_string(&pathbuf).await.ok()?;
        serde_json::from_str(&contents).ok()
    }


    // find_all traverses the directly picking up on the files and returning a vec
    async fn find_all(&mut self) -> Vec<M> {
        let mut values = Vec::<M>::new();

        let mut entries = match tokio::fs::read_dir(&self.collection_path).await {
            Ok(e) => e,
            Err(_e) => return values,
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let pathbuf = self.collection_path.clone().join(entry.file_name());
           
            if let Ok(contents) = tokio::fs::read_to_string(&pathbuf).await {
                let data = serde_json::from_str::<M>(&contents).ok();
                if let Some(value) = data {
                    values.push(value);
                }
            };
        }
        values
    }

    // update updates the json file for the id
    async fn update(&mut self, model: M) -> Result<()> {
        let pathbuf= utils::build_json_file_path(&self.collection_path, model.id());
        let json = serde_json::to_string_pretty(&model)?;
        fs::write(&pathbuf, json)
            .await
            .with_context(|| FsRepositoryError::FileCreation {path: pathbuf})?;
        Ok(())
    }


}


#[cfg(test)]
mod tests {

    use super::*;
    use once_cell::sync::Lazy;
    use serde::Deserialize;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct TestUser {
        id: String,
        name: String,
    }

    impl RepoModel<String> for TestUser {
        fn id(&self) -> String {
            self.id.clone()
        }
        fn collection(&self) -> &'static str {
            "user"
        }
    }

    static USER1: Lazy<TestUser> = Lazy::new(|| TestUser {
        id: "1".to_string(),
        name: "Test1".to_string(),
    });
    static USER2: Lazy<TestUser> = Lazy::new(|| TestUser {
        id: "2".to_string(),
        name: "Test1".to_string(),
    });

    #[tokio::test]
    async fn test_insert_1() {
        let pb = PathBuf::from("data/tests/users");
        let mut repo = FsRepository::<String, TestUser>::new("users".to_string(), pb);
        let user1 = &*USER1;
        repo.insert(user1.clone())
            .await
            .expect("Failed to create user");

        let user2 = &*USER2;
        repo.insert(user2.clone())
            .await
            .expect("Failed to create user");
    }

    #[tokio::test]
    async fn test_find_all() {
        let pb = PathBuf::from("data/tests1/users");
        let mut repo = FsRepository::<String, TestUser>::new("users".to_string(), pb);
        let values = repo.find_all().await;
        assert_eq!(values.len(), 2);

    }
}
