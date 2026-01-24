use serde::{Deserialize, Serialize};
use std::sync::Arc;
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
    account_id: String,
}

impl Account {
    pub fn new(user_id: String, account_id: String) -> Self {
        let id = [user_id.as_str(), "-", account_id.as_str()].concat();
        Account {
            id,
            user_id,
            account_id,
        }
    }
}

impl RepoModel<String> for Account {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn collection(&self) -> &'static str {
        "account"
    }
}

pub(crate) struct Service {
    pub db: Arc<Mutex<FsDatabase>>,
}

impl Service {
    pub fn new(db: Mutex<FsDatabase>) -> Self {
        Service { db: Arc::new(db) }
    }
}
