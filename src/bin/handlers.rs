use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, body::BoxBody,
    http::header::ContentType, web,
};
use serde::Serialize;
#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}
impl Responder for MyObj {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
async fn index(rep: HttpRequest) -> MyObj {
    MyObj { name: "hi" }
}
use actix_web::{Error, get};
use futures::{future::ok, stream::once};
#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"testasdfasdsfad")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
use actix_web::Either;
type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;
#[get("/either")]
async fn either() -> RegisterResult {
    if true {
        Either::Left(HttpResponse::BadRequest().body("bad data"))
    } else {
        Either::Right(Ok("Hello!"))
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(index))
            .service(stream)
            .service(either)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
