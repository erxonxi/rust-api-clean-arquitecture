use async_trait::async_trait;
use bson::{doc, Document};
use mongodb::{bson, Client};

use crate::internal::user::{ErrorsUserRepository, User, UserId, UserRepository};
use super::mongo::MongoClientFactory;

#[derive(Debug)]
pub struct MongoUserRepository {
    client: Client,
}

impl MongoUserRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn factory() -> Self {
        let client = MongoClientFactory::new("mongodb://localhost:27017".into()).await.unwrap();
        Self { client }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn save(&self, user: User) -> Result<(), ErrorsUserRepository> {
        let db = self.client.database("rust-mooc");
        let collection = db.collection::<Document>("users");
        let doc =
            doc! {"_id": user.id.value, "email": user.email.value, "password": user.password.value};
        if collection.insert_one(doc, None).await.is_err() {
            Err(ErrorsUserRepository::ErrorOnSave)
        } else {
            Ok(())
        }
    }

    async fn get(&self, id: UserId) -> Result<User, ErrorsUserRepository> {
        let db = self.client.database("rust-mooc");
        let collection = db.collection::<Document>("users");
        if let Ok(doc) = collection.find_one(None, None).await {
            Ok(User::from_doc(doc.unwrap()))
        } else {
            Err(ErrorsUserRepository::ErrorOnSave)
        }
    }
}
