use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum ErrorsUserId {
    InvalidUuid,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserId {
    pub value: String,
}

impl UserId {
    pub fn new(value: String) -> Result<Self, ErrorsUserId> {
        let uuid = Uuid::parse_str(&value);
        if let Ok(_) = uuid {
            Ok(Self { value })
        } else {
            Err(ErrorsUserId::InvalidUuid)
        }
    }

    pub fn _random() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorsUserEmail {
    Invalid,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserEmail {
    pub value: String,
}

impl UserEmail {
    pub fn new(value: String) -> Result<Self, ErrorsUserEmail> {
        let re =
            Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
                .unwrap();
        if re.is_match(&value) {
            Ok(Self { value })
        } else {
            Err(ErrorsUserEmail::Invalid)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserPassword {
    pub value: String,
}

impl UserPassword {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: UserId,
    pub email: UserEmail,
    pub password: UserPassword,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserPrimitives {
    pub id: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: UserId, email: UserEmail, password: UserPassword) -> Self {
        Self {
            id,
            email,
            password,
        }
    }

    pub fn from_primitives(user: UserPrimitives) -> Self {
        Self {
            id: UserId::new(user.id).unwrap(),
            email: UserEmail::new(user.email).unwrap(),
            password: UserPassword::new(user.password),
        }
    }

    pub fn to_primitives(&self) -> UserPrimitives {
        UserPrimitives {
            id: self.id.value.to_string(),
            email: self.email.value.to_string(),
            password: self.password.value.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorsUserRepository {
    ErrorOnSave,
}

#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
    async fn save(&self, user: User) -> Result<(), ErrorsUserRepository>;
    async fn get(&self, id: UserId) -> Result<User, ErrorsUserRepository>;
}

#[cfg(test)]
pub mod mothers {
    use fake::faker::internet::en::{FreeEmail, Password};
    use fake::Fake;

    use super::*;

    pub struct UserMother {}
    impl UserMother {
        pub fn random() -> User {
            let email = UserEmail::new(FreeEmail().fake());
            let password = UserPassword::new(Password(10..20).fake());
            User::new(UserId::_random(), email.unwrap(), password)
        }
    }
}

#[cfg(test)]
pub mod mocks {
    use super::{ErrorsUserRepository, User, UserId, UserRepository};

    #[derive(Debug)]
    pub struct MockUserRepository {
        fake_db: Vec<User>,
    }

    impl MockUserRepository {
        pub fn new() -> Self {
            Self { fake_db: vec![] }
        }

        pub fn with_db(fake_db: Vec<User>) -> Self {
            Self { fake_db }
        }
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn save(&self, _user: User) -> Result<(), ErrorsUserRepository> {
            Ok(())
        }

        async fn get(&self, id: UserId) -> Result<User, ErrorsUserRepository> {
            let user = self.fake_db.iter().find(|&u| u.id == id).unwrap();
            Ok(user.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_id() {
        let id = UserId::new("67e55044-10b1-426f-9247-bb680e5fe0c8".into());
        assert_eq!(id.unwrap().value, "67e55044-10b1-426f-9247-bb680e5fe0c8");
    }

    #[test]
    fn invalid_user_id() {
        let id = UserId::new("743h98g83048hew48etw340".into());
        assert_eq!(id.is_err(), true);
    }

    #[test]
    fn create_user_email() {
        let email = UserEmail::new("rubenruizpedreira@gmail.com".into());
        assert_eq!(email.unwrap().value, "rubenruizpedreira@gmail.com");
    }

    #[test]
    fn invalid_user_email() {
        let email = UserEmail::new("asdfthursrugx".into());
        assert_eq!(email.is_err(), true);
    }

    #[test]
    fn create_user_password() {
        let password = UserPassword::new("6284349".into());
        assert_eq!(password.value, "6284349")
    }

    #[test]
    fn create_user() {
        let id = UserId::new("67e55044-10b1-426f-9247-bb680e5fe0c8".into());
        let email = UserEmail::new("rubenruizpedreira@gmail.com".into());
        let password = UserPassword::new("6284349".into());
        let user = User::new(id.unwrap(), email.unwrap(), password);

        assert_eq!(user.id.value, "67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(user.email.value, "rubenruizpedreira@gmail.com");
        assert_eq!(user.password.value, "6284349");
    }
}
