pub mod status;
pub mod users;

use std::sync::Arc;

use crate::config::Config;
use crate::internal::plataform::storge::user_repository::MongoUserRepository;
use crate::internal::users::create::CreateUser;
use crate::internal::users::delete::DeleteUser;
use crate::internal::users::get::GetUser;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
pub async fn build() -> std::io::Result<()> {
    let config = Config::env();
    let user_repository = Arc::new(MongoUserRepository::new(config.mongo_url.to_owned()).await);
    let create_user = CreateUser::new(user_repository.clone());
    let get_user = GetUser::new(user_repository.clone());
    let delete_user = DeleteUser::new(user_repository.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(RestContainer {
                _config: config.to_owned(),
                create_user: create_user.to_owned(),
                get_user: get_user.to_owned(),
                delete_user: delete_user.to_owned(),
            }))
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/status").route(web::get().to(status::healthz)))
        .service(web::resource("/users").route(web::put().to(users::create_user)))
        .service(
            web::resource("/users/{id}")
                .route(web::get().to(users::get_user))
                .route(web::delete().to(users::delete_user)),
        );
}

#[derive(Debug)]
pub struct RestContainer {
    _config: Config,
    create_user: CreateUser,
    get_user: GetUser,
    delete_user: DeleteUser,
}
