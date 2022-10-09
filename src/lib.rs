use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
  let server = HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
    .listen(listener)?
    .run();

  Ok(server)
}
