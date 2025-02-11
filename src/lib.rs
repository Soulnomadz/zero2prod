use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
	    .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8000")?
    .run();
    //.await

    Ok(server)
}

//#[cfg(test)]
//mod tests {
//  use crate::health_check;
//
//  #[tokio::test]
//  async fn health_check_succeeds() {
//    let response = health_check().await;
//
//    assert!(response.status().is_success())
//  }
//}
