use std::fmt::Debug;

use async_trait::async_trait;
use regex::Regex;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum ErrorsUserId {
    InvalidUuid,
}

#[derive(Debug, PartialEq)]
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

    pub fn random() {
        Self { value: Uuid::new_v4().to_string() };
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorsUserEmail {
    Invalid,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct UserPassword {
    pub value: String,
}

impl UserPassword {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct User {
    pub id: UserId,
    pub email: UserEmail,
    pub password: UserPassword,
}

impl User {
    pub fn new(id: UserId, email: UserEmail, password: UserPassword) -> Self {
        Self {
            id,
            email,
            password,
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
