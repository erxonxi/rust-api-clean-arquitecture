use async_trait::async_trait;
use bson::{doc, Document};
use mongodb::{bson, Collection};

use super::mongo::MongoRepository;
use crate::internal::user::{ErrorsUserRepository, User, UserId, UserPrimitives, UserRepository};

#[derive(Debug)]
pub struct MongoUserRepository {
    collection: Collection<Document>,
}

impl MongoUserRepository {
    pub async fn new(url: String) -> Self {
        let collection = Self::get_collection(url, "rust-mooc".into(), "users".into()).await;
        Self { collection }
    }
}

impl MongoRepository for MongoUserRepository {
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn save(&self, user: User) -> Result<(), ErrorsUserRepository> {
        let doc =
            doc! {"_id": user.id.value, "email": user.email.value, "password": user.password.value};
        if self.collection.insert_one(doc, None).await.is_err() {
            Err(ErrorsUserRepository::ErrorOnSave)
        } else {
            Ok(())
        }
    }

    async fn get(&self, id: UserId) -> Result<User, ErrorsUserRepository> {
        if let Ok(doc) = self
            .collection
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
            Err(ErrorsUserRepository::UserNotFound)
        }
    }

    async fn delete(&self, id: UserId) -> Result<(), ErrorsUserRepository> {
        if self
            .collection
            .delete_one(doc! { "_id": id.value }, None)
            .await
            .is_err()
        {
            Err(ErrorsUserRepository::UserNotFound)
        } else {
            Ok(())
        }
    }
}
