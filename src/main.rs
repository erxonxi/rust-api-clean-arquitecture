mod internal;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use crate::internal::user::{User, UserEmail, UserId, UserPassword, UserRepository};
use internal::plataform::storge::mongo::MongoClientFactory;
use internal::plataform::storge::user_repository::MongoUserRepository;

#[get("/")]
async fn hello() -> impl Responder {
    let client = MongoClientFactory::new("mongodb://localhost:27017".into()).await;
    let clien = client.unwrap();
    let repository = MongoUserRepository::new(&clien);
    let id = UserId::new("67e55044-10b1-426f-9247-bb680e5fe0c8".into());
    let email = UserEmail::new("rubenruizpedreira@gmail.com".into());
    let password = UserPassword::new("6284349".into());
    let user = User::new(id.unwrap(), email.unwrap(), password);
    repository.save(user).await;

    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Deserialize)]
struct CreateUserBody {
    name: String,
    password: String,
}

#[post("/users")]
async fn create_user(body: web::Json<CreateUserBody>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo).service(create_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
