use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};
use tracing::debug;

use crate::{
    core::{Initializable, RepoKey, RepoModel, Repository},
    fs::{
        collections::CollectionMetadata, errors::FsDatabaseError, repository::FsRepository, utils,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FsDatabase {
    name: String,
    file_path: String,
    collections: HashMap<String, CollectionMetadata>,

    #[serde(skip)] // Don't serialize this field!
    repos: HashMap<String, Box<dyn Initializable + Send + Sync>>,
}

impl FsDatabase {
    pub async fn new(name: String, file_path: String) -> Result<Self> {
        let mut db = FsDatabase::load_from_file(&name, &file_path)?;
        db.initialize().await?;
        Ok(db)
    }

    async fn initialize(&mut self) -> Result<()> {
        // for (name, _) in self.collections.iter_mut() {
        //     debug!("Initializing collection: {}", name);
        //     if let Some(repo)  = self.nrepos.get_mut(name) {
        //         // repo.init().await?;
        //     } else {
        //         debug!("Not found");
        //     }

        // }
        Ok(())
    }
    // load database info from file
    fn load_from_file(name: &str, file_path: &str) -> Result<Self> {
        let pathbuf = utils::build_json_file_path(&PathBuf::from(&file_path), name);

        debug!("Loading database from {:?}", pathbuf);
        if Path::new(&pathbuf).exists() {
            let contents = fs::read_to_string(&pathbuf)?;
            debug!("contents: {:?}", contents);
            Ok(serde_json::from_str(&contents)?)
        } else {
            debug!("File does not exist");
            Ok(Self {
                name: name.to_string(),
                file_path: file_path.to_string(),
                collections: HashMap::new(),
                repos: HashMap::new(),
            })
        }
    }

    // save database info to file
    async fn save_to_file(&self) -> Result<()> {
        let pathbuf =
            utils::build_json_file_path(&PathBuf::from(&self.file_path), self.name.clone());
        debug!("Saving Database to {:?}", pathbuf);
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(&pathbuf, json)?;
        Ok(())
    }

    // reguster_collection check if the collection exists, creates it if it does not
    pub async fn register_collection<K, M>(&mut self, name: String) -> Result<()>
    where
        K: RepoKey,
        M: RepoModel<K>,
    {
        let full_path = PathBuf::from(&self.file_path).join(&name);

        if !self.collections.contains_key(&name) {
            let metadata = CollectionMetadata { name: name.clone() };
            let _ = self.collections.insert(name.clone(), metadata);
            let _ = fs::create_dir_all(full_path.clone());
            self.save_to_file().await?;
        }

        let mut repository = FsRepository::<K, M>::new(name.clone(), full_path)?;
        repository.initialize().await?;
        self.repos
            .entry(name.clone())
            .insert_entry(Box::new(repository));

        Ok(())
    }

    // collection check if the collection exists, errors if it does not
    pub async fn collection<K, M>(&mut self, name: String) -> Result<&mut dyn Repository<K, M>>
    where
        K: RepoKey,
        M: RepoModel<K>,
    {
        if !self.repos.contains_key(&name) {
            return Err(anyhow::anyhow!(
                FsDatabaseError::CollectionRepoisitoryMissingError {
                    path: name.clone().into()
                }
            ));
        }

        let v = self
            .repos
            .get_mut(&name)
            .ok_or(FsDatabaseError::CollectionRespositoryError {
                path: name.clone().into(),
            });

        let any: &mut dyn Any = match v {
            Ok(v) => v.as_any_mut(),
            Err(_e) => {
                return Err(anyhow::anyhow!(
                    FsDatabaseError::CollectionRepoisitoryMissingError {
                        path: name.clone().into()
                    }
                ));
            }
        };

        let repo: &mut FsRepository<K, M> = any
            .downcast_mut::<FsRepository<K, M>>()
            .context(FsDatabaseError::CollectionRepoisitoryDowncastError { path: name.into() })?;

        Ok(repo)
    }
}
