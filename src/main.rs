use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use crate::internal::application::service::sandwich_service::Service;
use crate::internal::infrastructure::rest::create_sandwich::create_sandwich;
use crate::internal::infrastructure::rest::get_sandwich::get_sandwich;

mod internal;
mod test_shared;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let service = Service::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // .app_data(Data::new(service.clone()))
            // .route("/healthz", web::get().to(HttpResponse::Ok))
            // .service(
            //     web::scope("/recipes")
            //         .service(web::resource("sandwich/{id}")
            //             .route(web::get()).to(get_sandwich::<Service>))
            //         .service(web::resource("sandwich")
            //             .route(web::post()).to(create_sandwich::<Service>))
            // )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

