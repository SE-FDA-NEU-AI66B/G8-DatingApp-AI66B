use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::cell::Cell;
use std::sync::{atomic::AtomicUsize, Arc};
#[derive(Clone, Debug)]
pub struct Database(pub Arc<Pool<Postgres>>);
