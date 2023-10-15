use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;

use crate::internal::application::service::sandwich_service::Service;
use crate::internal::infrastructure::rest::create_sandwich::create_sandwich;
use crate::internal::infrastructure::rest::get_sandwich::get_sandwich;
use crate::internal::infrastructure::store::config::{connect_to_db};
use crate::internal::infrastructure::store::sandwich_store::SandwichStore;

mod internal;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let sql_lite = connect_to_db().await;
    let sandwich_store= SandwichStore::new(sql_lite);
    let sandwich_service = Service::new(sandwich_store);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(sandwich_service.clone()))
            .route("/healthz", web::get().to(HttpResponse::Ok))
            .route("/sandwich", web::post().to(create_sandwich::<Service<SandwichStore>>))
            .route("/sandwich/{id}", web::get().to(get_sandwich::<Service<SandwichStore>>))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

