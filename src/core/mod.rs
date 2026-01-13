use anyhow::Result;
use async_trait::async_trait;
pub trait RepoModel<K> {
    
    // Returns the id of the model;
    fn id(&self) -> K;

    // Returns the name of the collection;
    fn collection(&self) ->  &'static str;

}



#[async_trait]
pub trait Repository<K, M>:Send{

    async fn insert(&mut self, repo: M) -> Result<()>; 
    async fn delete(&mut self, id: K) -> Result<()>; 
    async fn find_by_id(&mut self, id: K) -> Option<M>;
    async fn find_all(&mut self) -> Vec<M>;
    async fn update(&mut self, repo: M) -> Result<()>; 
    // async fn find_all(&mut self) -> Result<()>;
}



// pub trait RepoModel {
//     type Id: Send + Sync + Clone + Eq;

//     // Returns the id of the model;
//     fn id(&self) -> Self::Id;

//     // fn set_id(&mut self, id: K);

//     // Returns the name of the collection;
//     fn collection(&self) ->  &'static str;

// }
