use async_trait::async_trait;
use bson::{doc, Document};
use mongodb::{bson, Client};

use super::mongo::MongoClientFactory;
use crate::internal::user::{ErrorsUserRepository, User, UserId, UserPrimitives, UserRepository};

#[derive(Debug)]
pub struct MongoUserRepository {
    client: Client,
}

impl MongoUserRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn factory() -> Self {
        let client = MongoClientFactory::new("mongodb://localhost:27017".into())
            .await
            .unwrap();
        Self::new(client)
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
        if let Ok(doc) = collection
            .find_one(
                doc! {
                      "_id": &id.value
                },
                None,
            )
            .await
        {
            let user_doc = doc.unwrap();
            Ok(User::from_primitives(UserPrimitives {
                id: user_doc.get_str("_id").unwrap().to_string(),
                email: user_doc.get_str("email").unwrap().to_string(),
                password: user_doc.get_str("password").unwrap().to_string(),
            }))
        } else {
            Err(ErrorsUserRepository::ErrorOnSave)
        }
    }
}
