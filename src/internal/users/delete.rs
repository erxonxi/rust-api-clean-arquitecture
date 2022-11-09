use std::sync::Arc;

use crate::internal::user::{ErrorsUserRepository, UserId, UserRepository};

#[derive(Clone, Debug)]
pub struct DeleteUser {
    repository: Arc<dyn UserRepository>,
}

impl DeleteUser {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn run(&self, id: UserId) -> Result<(), ErrorsUserRepository> {
        self.repository.delete(id).await
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

        let mock = MockUserRepository::new();
        let service = DeleteUser::new(Arc::new(mock));

        assert_eq!(Ok(()), service.run(user.id).await);
    }
}
