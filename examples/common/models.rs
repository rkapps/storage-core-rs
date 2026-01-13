use std::{fmt::{Debug, Display}, hash::Hash, sync::Arc};

use anyhow::Result;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use storage_core::{core::RepoModel, fs::database::FsDatabase};
use tokio::sync::Mutex;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl RepoModel<String> for User {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn collection(&self) -> &'static str {
        "user"
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub id: String,
    user_id: String,
    account_id: String
}

impl Account {

    pub fn new(user_id: String, account_id: String) -> Self {
        let id = [user_id.as_str(), "-", account_id.as_str()].concat();
        Account{id, user_id, account_id}
    }
}

impl RepoModel<String> for Account {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn collection(&self) ->  &'static str {
        "account"
    }
}



pub struct Storage {
    pub db: FsDatabase,
    collections: Vec<String>
}

impl Storage {

    pub fn new(db: String, file_path: String) -> Self {
        Storage{db: FsDatabase::new(db, file_path), collections: Vec::new()}
    }

    pub fn register_collection<K, M>(&mut self, name: String) -> Result<()> 
    where
        K: Eq + Hash + Send + Clone + Debug + Display + 'static,
        M: RepoModel<K> + Send + Clone + Debug + Serialize + 'static + DeserializeOwned {
            
        let _ = self.db.collection::<K,M>(name); 
        Ok(())
    }
}


pub struct Service {
    pub storage: Arc<Mutex<Storage>>
}

impl Service {
    pub fn new(storage: Mutex<Storage>) -> Self{
        Service{storage : Arc::new(storage)}
    }
}