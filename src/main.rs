use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use std::path::PathBuf;
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("whatsapp")
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("req_body")
}
async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    // let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open("assets/templates/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
    // use actix_web::{ss}
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("cert/rsa_4096.key", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_file("cert/rsa_4096.crt", SslFiletype::PEM)
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .route("/login", web::get().to(index))
    })
    .bind_openssl(("0.0.0.0", 8080), builder)?
    .run()
    .await
}
