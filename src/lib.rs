use actix_web::{web, App, Responder, HttpRequest, HttpServer, HttpResponse};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:9090")?
        .run()
        .await
}
