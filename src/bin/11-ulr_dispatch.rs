use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}
async fn index_r() -> HttpResponse {
    HttpResponse::Ok().body("Hello from resource")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::guard;
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/user/{user_name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(index)),
            )
            .route("/", web::get().to(index))
            .route("/user", web::post().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
