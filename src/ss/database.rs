use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub fn get_db_uri() -> String {
    use std::env;
    format!(
        "postgres://postgres:{}@localhost/userdb",
        env::var_os("PGPASS")
            .map(|i| i.into_string().unwrap())
            .unwrap_or("".to_string())
    )
}
pub async fn get_database_pool() -> Arc<Pool<Postgres>> {
    Arc::new(
        PgPoolOptions::new()
            .max_connections(100)
            .connect(&get_db_uri())
            .await
            .unwrap(),
    )
}
use actix_web::web::Data;
use std::cell::Cell;
// pub fn config(database_pool: Arc<Pool<Postgres>>) {
//     let data = crate::lib::share::database::Database(database_pool);
//     // cfg: &mut actix_web::web::ServiceConfig,
//     //     cfg.app_data(Data::new(data));
// }
