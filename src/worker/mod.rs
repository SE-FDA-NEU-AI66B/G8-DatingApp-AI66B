use actix_web::web;
use serde::Deserialize;
use std::cell::Cell;
use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
#[derive(Clone, Deserialize)]
pub struct Worker {
    worker_count: usize,
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

// use actix_web::HttpResponse;
pub fn config(cfg: &mut web::ServiceConfig) {
    static WORKER: AtomicUsize = AtomicUsize::new(0);
    let mut data = Worker {
        worker_count: WORKER.load(Ordering::Relaxed),
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };
    data.global_count.fetch_add(1, Ordering::Relaxed);
    cfg.app_data(data);
}
