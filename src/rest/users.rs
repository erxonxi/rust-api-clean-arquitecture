use crate::{
    internal::user::{UserEmail, UserId, UserPassword},
    rest::RestContainer,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserBody {
    id: String,
    email: String,
    password: String,
}

pub async fn create_user(
    data: web::Data<RestContainer>,
    body: web::Json<CreateUserBody>,
) -> impl Responder {
    match UserId::new(body.id.to_string()) {
        Ok(id) => match UserEmail::new(body.email.to_string()) {
            Ok(email) => {
                let password = UserPassword::new(body.password.to_string());
                match data.create_user.run(id, email, password).await {
                    Err(_) => {
                        return HttpResponse::InternalServerError().json("Internal server error")
                    }
                    _ => (),
                }
            }
            Err(_) => return HttpResponse::NotAcceptable().json("Invalid user email"),
        },
        Err(_) => return HttpResponse::NotAcceptable().json("Invalid user id"),
    }

    HttpResponse::Ok().json("Ok")
}
