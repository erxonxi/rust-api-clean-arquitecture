use std::sync::Arc;

use crate::internal::user::{
    ErrorsUserRepository, User, UserId, UserRepository,
};

#[derive(Clone, Debug)]
pub struct GetUser {
    repository: Arc<dyn UserRepository>,
}

impl GetUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn run(&self, id: UserId) -> Result<User, ErrorsUserRepository> {
        self.repository.get(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::user::mocks::MockUserRepository;
    use crate::internal::user::mothers::UserMother;

    #[actix_rt::test]
    async fn get_user_valid_data() {
        let user = UserMother::random();
        let user_id = user.id.clone();

        let mock = MockUserRepository::with_db(vec![user.clone()]);
        let service = GetUser::new(Arc::new(mock));

        assert_eq!(Ok(user), service.run(user_id).await);
    }
}
