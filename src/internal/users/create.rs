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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::user::mocks::MockUserRepository;
    use crate::internal::user::mothers::UserMother;

    #[actix_rt::test]
    async fn create_user_valid_data() {
        let mock = MockUserRepository::new();
        let user = UserMother::random();
        let service = CreateUser::new(Arc::new(mock));

        assert_eq!(
            Ok(()),
            service.run(user.id, user.email, user.password).await
        );
    }
}
