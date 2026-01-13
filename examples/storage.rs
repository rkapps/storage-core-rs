mod common;

use anyhow::Result;

use crate::common::models::{Account, Storage, User};


#[tokio::main]
async fn main() -> Result<()>{

    let mut storage = Storage::new("mystore".to_string(), "data/mystoredb".to_string());
    storage.db.save_to_file()?;

    {
        let _ = storage.register_collection::<String, User>("user".to_string())?;
        let urepo = storage.db.collection("user".to_string())?;
        let user1 = User{id: "1".to_string(), name: "storage_test1".to_string()};    
        let user2 = User{id: "2".to_string(), name: "storage_test2".to_string()};
        urepo.insert(user1).await?;
        urepo.insert(user2).await?;

        let users = urepo.find_all().await;
        println!("User count {:?}", users.len());
    }

    {

        let _ = storage.register_collection::<String, Account>("account".to_string())?;
        let arepo = storage.db.collection("account".to_string())?;
        
        let account1 = Account::new("1".to_string(), "1".to_string());
        let account2 = Account::new("2".to_string(), "2".to_string());
    
        arepo.insert(account1).await?;
        arepo.insert(account2).await?;
    
        let accounts = arepo.find_all().await;
        println!("account count {:?}", accounts.len());
    
    }

    Ok(())
}
