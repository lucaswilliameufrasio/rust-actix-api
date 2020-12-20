use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::env;

async fn index() -> HttpResponse {
    #[derive(Debug, Serialize, Deserialize)]
    struct Response {
        id: u64,
        message: String,
    }
    let response = Response {
        id: 1,
        message: String::from("Hello World"),
    };
    // println!("model: {:?}", &item);
    HttpResponse::Ok().json(response) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", "0.0.0.0", port);

    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind(address)?
        .run()
        .await
}
