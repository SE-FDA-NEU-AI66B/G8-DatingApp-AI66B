use actix_web::error::ReadlinesError;
use actix_web::{Result, error, error::Error, get};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display("internal error")]
    InternalError,
    #[display("bad request")]
    BadClientData,
    #[display("timeout")]
    Timeout,
}
#[derive(Debug, Display, Error)]
struct MyError2 {
    name: &'static str,
}
use actix_web::http::StatusCode;
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::OK,
        }
    }
}
impl error::ResponseError for MyError2 {}
#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    Err(MyError::InternalError)
}
#[get("/2")]
async fn index2() -> actix_web::Result<String> {
    Err(MyError2 { name: "Asdfsdf" }).map_err(|err| error::ErrorExpectationFailed(err.name))
}
#[derive(Debug, Display, Error)]
enum UserError {
    #[display("Validation error on field: {field}")]
    ValidationError { field: String },
}
use actix_web::{HttpResponse, http::header::ContentType};
impl error::ResponseError for UserError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
#[get("/")]
async fn page_error() -> Result<&'static str, UserError> {
    Ok("success!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    use actix_web::{App, HttpServer, middleware::Logger, web};
    env_logger::init();
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().service(index).wrap(logger).service(page_error)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
