use crate::{
    internal::user::{UserEmail, UserId, UserPassword},
    rest::RestContainer,
};
use actix_web::{web, HttpResponse, Responder};

pub async fn create_user(data: web::Data<RestContainer>) -> impl Responder {
    let id = UserId::new("67e55044-10b1-426f-9247-bb680e5fe0c8".into());
    let email = UserEmail::new("rubenruizpedreira@gmail.com".into());
    let password = UserPassword::new("6284349".into());
    data.create_user
        .run(id.unwrap(), email.unwrap(), password)
        .await;
    HttpResponse::Ok().json("Ok")
}
