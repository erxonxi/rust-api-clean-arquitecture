pub mod status;
pub mod users;

use std::sync::Arc;

use crate::internal::plataform::storge::user_repository::MongoUserRepository;
use crate::internal::users::create::CreateUser;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
pub async fn build() -> std::io::Result<()> {
    let user_repository = Arc::new(MongoUserRepository::factory().await);
    let create_user = CreateUser::new(user_repository);
    HttpServer::new(move || {
        App::new()
            .data(RestContainer {
                create_user: create_user.to_owned(),
            })
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/status").route(web::get().to(status::healthz)))
        .service(web::resource("/users").route(web::put().to(users::create_user)));
}

#[derive(Debug)]
pub struct RestContainer {
    create_user: CreateUser,
}