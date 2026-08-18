use actix_web::{App, HttpServer, Responder, Result, get, post, web};
#[get("/user/{uid}/{friend}")]
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (uid, friend) = path.into_inner();
    Ok(format!("Welcome {}, uid {}!", friend, uid))
}
use serde::Deserialize;
#[derive(Deserialize)]
struct Info {
    uid: u32,
    friend: String,
}
#[get("/user2/{uid}/{friend}")]
async fn index2(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome2 {},user_id {}", info.friend, info.uid))
}
#[derive(Deserialize)]
struct FromData {
    username: String,
}
#[post("/maybe")]
async fn maybe(form: Option<web::Form<FromData>>) -> Result<String> {
    let Some(form) = form else {
        return Ok("missing or invalid form data".to_string());
    };
    Ok(format!("Welcome {}!", form.username))
}
use std::{
    cell::Cell,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
#[derive(Clone)]
struct AppState {
    worker_count: Cell<usize>,
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!(
        "worker_count:{}\nlocal_count:{}\nglobal_count:{}",
        data.worker_count.get(),
        data.local_count.get(),
        data.global_count.load(Ordering::Relaxed)
    )
}
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);
    data.local_count.set(data.local_count.get() + 1);
    show_count(data).await
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut data = AppState {
        worker_count: Cell::new(0),
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };
    let worker_count = Arc::new(AtomicUsize::new(0));
    HttpServer::new(move || {
        data.worker_count
            .set(worker_count.fetch_add(1, Ordering::Relaxed));

        // data.worker_count.fetch_add(1, Ordering::Relaxed);
        App::new()
            .app_data(web::Data::new(data.clone()))
            .route("/show", web::to(show_count))
            .route("/add", web::to(add_one))
            .service(index)
            .service(index2)
            .service(maybe)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
