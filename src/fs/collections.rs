use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionMetadata {
    pub name: String
}
