use std::sync::Arc;

use crate::internal::user::{
    ErrorsUserRepository, User, UserEmail, UserId, UserPassword, UserRepository,
};

#[derive(Clone, Debug)]
pub struct CreateUser {
    repository: Arc<dyn UserRepository>,
}

impl CreateUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn run(
        &self,
        id: UserId,
        email: UserEmail,
        password: UserPassword,
    ) -> Result<(), ErrorsUserRepository> {
        let user = User::new(id, email, password);
        self.repository.save(user).await
    }
}
