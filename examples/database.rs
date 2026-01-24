mod common;

use anyhow::Result;
use storage_core::fs::database::FsDatabase;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::common::models::{Account, User};

#[tokio::main]
async fn main() -> Result<()>{

    let filter = filter::Targets::new()
        .with_target("storage_core::examples", Level::DEBUG)
        .with_target("storage_core::fs", Level::DEBUG)
        ;
     tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty())  // Compact format
        .with(filter)
        .init();    


    let mut fsdb = FsDatabase::new("mystore".to_string(), "data/mystoredb".to_string()).await?;
    fsdb.register_collection::<String, User>("user".to_string()).await?;
  
    {
        let urepo = fsdb.collection::<String, User>("user".to_string()).await?;
    
        let user1 = User{id: "1".to_string(), name: "storage_test1".to_string()};    
        let user2 = User{id: "2".to_string(), name: "storage_test2".to_string()};
        let cuser1 = User{id: "1".to_string(), name: "storage_test1111111111".to_string()};
        urepo.insert(user1.clone()).await?;
        urepo.insert(user2).await?;
        urepo.find_by_id("1".to_string()).await;
        urepo.find_by_id("2".to_string()).await;
        urepo.update(cuser1).await?;
        urepo.find_by_id("1".to_string()).await;
        urepo.delete(user1).await?;

        let option = urepo.find_by_id("2".to_string()).await;
        println!("User {:?}", Some(option));
        let users = urepo.find_all().await;
        println!("User count {:?}", users);
    }    


    {

        let _ = fsdb.register_collection::<String, Account>("account".to_string()).await?;
        let arepo = fsdb.collection("account".to_string()).await?;
        
        let account1 = Account::new("1".to_string(), "1".to_string());
        let account2 = Account::new("2".to_string(), "2".to_string());
    
        arepo.insert(account1).await?;
        arepo.insert(account2).await?;
    
        let accounts = arepo.find_all().await;
        println!("account count {:?}", accounts.len());
    
    }    
    Ok(())        
}