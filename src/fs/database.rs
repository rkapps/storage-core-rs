use anyhow::{Context, Result};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
    fs,
    hash::Hash,
    path::{Path, PathBuf},
};
use tracing::debug;

use crate::{
    core::{RepoModel, Repository}, fs::{collections::CollectionMetadata, errors::FsDatabaseError, repository::FsRepository, utils}
};
#[derive(Debug, Serialize, Deserialize)]
pub struct FsDatabase {
    name: String,
    file_path: PathBuf,
    collections: HashMap<String, CollectionMetadata>,

    #[serde(skip)] // Don't serialize this field!
    repos: HashMap<String, Box<dyn Any + Send>>,
}

impl FsDatabase {
    pub fn new(name: String, file_path: String) -> Self {
        Self {
            name,
            file_path: PathBuf::from(file_path),
            collections: HashMap::new(),
            repos: HashMap::new(),
        }
    }

    // load database info from file
    pub fn load_from_file(name: String, file_path: String) -> Result<Self> {
        let pathbuf= utils::build_json_file_path(&PathBuf::from(&file_path), name.clone());

        debug!("Full path {:?}", pathbuf);
        if Path::new(&pathbuf).exists() {
            let contents = fs::read_to_string(&pathbuf)?;
            debug!("contents: {:?}", contents);
            Ok(serde_json::from_str(&contents)?)
        } else {
            debug!("File does not exist");
            Ok(Self::new(name, file_path))
        }
    }

    // save database info to file
    pub fn save_to_file(&self) -> Result<()> {
        let pathbuf= utils::build_json_file_path(&PathBuf::from(&self.file_path), self.name.clone());
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(&pathbuf, json)?;
        Ok(())
    }

    // collection check if the collection exists, creates it if it does not and returns the repository
    pub fn collection<K, M>(&mut self, name: String) -> Result<&mut dyn Repository<K, M>>
    where
        K: Eq + Hash + Send + Clone + Debug + Display + 'static,
        M: RepoModel<K> + Send + Clone + Debug + Serialize + 'static + DeserializeOwned,
    {
        let full_path = self.file_path.join(&name);

        if !self.collections.contains_key(&name) {
            let metadata = CollectionMetadata { name: name.clone() };
            let _ = self.collections.insert(name.clone(), metadata);
            let _ = fs::create_dir_all(full_path.clone());
            self.save_to_file()?;
        }

        if !self.repos.contains_key(&name) {
            let respository = FsRepository::<K, M>::new(name.clone(), full_path);
            self.repos.insert(name.clone(), Box::new(respository));
        }

        let v = self.repos.get_mut(&name).ok_or(
        FsDatabaseError::CollectionRespositoryError { path: name.clone().into() }
        );

        let any = match v {
            Ok(v) => v,
            Err(_e) => {
                return Err(anyhow::anyhow!(FsDatabaseError::CollectionRepoisitoryMissingError { path: name.clone().into() }));
            }
        };

        let repo: &mut FsRepository<K, M> = any
            .downcast_mut::<FsRepository<K, M>>()
            .context(FsDatabaseError::CollectionRepoisitoryDowncastError{path: name.into()})?;

        Ok(repo)

    }
}
