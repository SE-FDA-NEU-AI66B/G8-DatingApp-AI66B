use actix_web::web::Data;
use leptos_actix::extract;
use sqlx::types::Uuid;
pub async fn user_id(cookie: &[u8]) -> Result<Uuid, String> {
    let db: Data<crate::share::database::Database> = extract().await.unwrap_or_else(|i| {
        println!("{:?}", i);
        panic!("asdf");
    });
    let db = (*db).0.as_ref();
    let userid: Result<(Uuid,), _> =
        sqlx::query_as("SELECT id from cookie_login where cookie=$1 and expires_at<now()")
            .bind(cookie.clone())
            .fetch_one(db)
            .await;
    let userid = userid.map(|i| i.0);
    userid.map_err(|err| err.to_string())
}
