use async_trait::async_trait;
use bson::{doc, Document};
use mongodb::{bson, Client};

use crate::internal::user::{ErrorsUserRepository, User, UserRepository};

pub struct MongoUserRepository<'a> {
    client: &'a Client,
}

#[async_trait]
impl<'a> UserRepository<'a, Client> for MongoUserRepository<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn save(&self, user: User) -> Result<(), ErrorsUserRepository> {
        let db = self.client.database("rust-mooc");
        let collection = db.collection::<Document>("books");
        let doc =
            doc! {"_id": user.id.value, "email": user.email.value, "password": user.password.value};
        if collection.insert_one(doc, None).await.is_err() {
            Err(ErrorsUserRepository::ErrorOnSave)
        } else {
            Ok(())
        }
    }
}
