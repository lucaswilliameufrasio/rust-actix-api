use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

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
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
