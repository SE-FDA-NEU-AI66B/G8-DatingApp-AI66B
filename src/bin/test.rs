#![feature(lock_value_accessors)]
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, get, post, web};
use std::time::{self, Duration};
#[get("/")]
async fn hello(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Hello {}!x{}", app_name, counter)
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("req_body")
}
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("req_body")
}
async fn index() -> impl Responder {
    HttpResponse::Ok().body("index")
}

use std::sync::Mutex;
struct AppState {
    counter: Mutex<i32>,
    app_name: String,
}
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test").route(web::get().to(|| async { HttpResponse::Ok().body("test") })),
    );
}
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
#[get("/user/{user_id}/{friend}")]
async fn extract(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();

    Ok(format!("Whlcom {}, user_id {}!", friend, user_id))
}

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let scope = web::scope("/user").service(show_user);
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("cert/rsa_4096.key", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_file("cert/rsa_4096.crt", SslFiletype::PEM)
        .unwrap();
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(scoped_config))
            .configure(config)
            // .app_data(web::Data::new(AppState {
            //     counter: Mutex::new(0),
            //     app_name: String::from("Atrix Web"),
            // }))
            // .service(hello)
            .service(echo)
            .service(extract)
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .keep_alive(Duration::from_secs(75))
    .shutdown_timeout(10)
    .bind_openssl(("0.0.0.0", 8080), builder)?
    .run()
    .await
}
