use anyhow::Result;
use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::marker::PhantomData;
use std::{fmt::Debug, path::PathBuf};
use tracing::debug;

use crate::core::{Initializable, RepoKey, RepoModel, Repository};
use crate::fs::file::{RECORD_TYPE_ACTIVE, RECORD_TYPE_DELETED, read_record, write_active_record};

#[derive(Debug)]
pub struct FsRepository<K, M>
where
    K: RepoKey,
    M: RepoModel<K>,
{
    pub name: String,
    collection_path: PathBuf,
    file: File,
    offsetm: HashMap<K, u64>,
    _phantom: PhantomData<(K, M)>,
}

impl<K, M> FsRepository<K, M>
where
    K: RepoKey,
    M: RepoModel<K>,
{
    pub fn new(name: String, collection_path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .create(true) // Create the file if it doesn't exist
            .append(true) // Open in append mode
            .open(FsRepository::<K, M>::file_path(&name, &collection_path))?;

        Ok(Self {
            name: name.clone(),
            collection_path,
            file,
            offsetm: HashMap::new(),
            _phantom: PhantomData,
        })
    }

    fn file_path(name: &str, collection_path: &PathBuf) -> PathBuf {
        collection_path.join(format!("{}.bin", &name))
    }
}

#[async_trait]
impl<K, M> Initializable for FsRepository<K, M>
where
    K: RepoKey,
    M: RepoModel<K>,
{
    async fn initialize(&mut self) -> Result<()> {
        let mut offset = self.file.seek(SeekFrom::Start(0))?;
        debug!("Initializing repo: {}...", self.name);
        loop {
            let (header, model) = match read_record::<M>(&mut self.file, offset) {
                Ok((header, model)) => (header, model),
                Err(e) => {
                    debug!("Read error: {}", e);
                    break;
                }
            };

            debug!("Record Type: {:?}", header.record_type);
            match header.record_type {
                RECORD_TYPE_ACTIVE => {
                    self.offsetm.insert(model.id(), offset);
                }
                RECORD_TYPE_DELETED => {
                    self.offsetm.remove(&model.id());
                }
                _ => {
                    break;
                }
            }
            offset = self.file.stream_position()?;
        }
        debug!("Initializing done.");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
    fn as_any(&mut self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[async_trait]
impl<K, M> Repository<K, M> for FsRepository<K, M>
where
    K: RepoKey,
    M: RepoModel<K>,
{
    // insert creates a new json file for the chat id. creates the directory structure too.
    async fn insert(&mut self, model: M) -> Result<()> {
        // // Create the directories if they do not exist
        // if !self.collection_path.exists() {
        //     let _ = fs::create_dir_all(&self.collection_path)
        //         .await
        //         .with_context(|| FsRepositoryError::DirectoryCreation {
        //             path: (self.collection_path.clone()),
        //         })?;
        // }
        let offset = write_active_record(&mut self.file, RECORD_TYPE_ACTIVE, &model, false)?;
        self.offsetm.insert(model.id(), offset);
        debug!("Insert id:{} at offset:{}", model.id(), offset);
        Ok(())
    }

    // delete appends the delete record
    async fn delete(&mut self, model: M) -> Result<()> {
        let _ = write_active_record(&mut self.file, RECORD_TYPE_DELETED, &model, false)?;
        self.offsetm.remove(&model.id());
        Ok(())
    }

    // find_by_id finds the json file for the id, marshalls that into the object
    async fn find_by_id(&mut self, id: K) -> Option<M> {
        let offset = self.offsetm.get(&id)?;
        debug!("Find_by_id Id:{} offset:{}", id, offset);
        let (_, model) = read_record::<M>(&mut self.file, *offset).ok()?;
        Some(model)
    }

    // find_all returns all values from offset map
    async fn find_all(&mut self) -> Vec<M> {
        let mut values = Vec::<M>::new();
        debug!("Find_all Offset map length: {}", self.offsetm.len());
        for offset in self.offsetm.values() {
            if let Some((_, model)) = read_record::<M>(&mut self.file, *offset).ok() {
                values.push(model);
            };
        }
        values
    }

    // update appends the udpated record
    async fn update(&mut self, model: M) -> Result<()> {
        let offset = write_active_record(&mut self.file, RECORD_TYPE_ACTIVE, &model, false)?;
        self.offsetm.insert(model.id(), offset);
        debug!("Update id:{} at offset:{}", model.id(), offset);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use once_cell::sync::Lazy;
    use serde::{Deserialize, Serialize};

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
    async fn test_insert_1() -> Result<()> {
        let pb = PathBuf::from("data/tests/users");
        let mut repo = FsRepository::<String, TestUser>::new("users".to_string(), pb)?;
        let user1 = &*USER1;
        repo.insert(user1.clone())
            .await
            .expect("Failed to create user");

        let user2 = &*USER2;
        repo.insert(user2.clone())
            .await
            .expect("Failed to create user");

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all() -> Result<()> {
        let pb = PathBuf::from("data/tests/users");
        let mut repo = FsRepository::<String, TestUser>::new("users".to_string(), pb)?;
        repo.initialize().await?;
        let values = repo.find_all().await;
        println!("{}", values.len());
        // assert_eq!(values.len(), 2);
        Ok(())
    }
}
