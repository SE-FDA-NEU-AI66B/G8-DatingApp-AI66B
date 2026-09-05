use actix_web::web::Data;
use serde::Deserialize;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    use actix_web::web;

    static WORKER: AtomicUsize = AtomicUsize::new(0);
    let data = crate::lib::share::worker::Worker {
        worker_count: WORKER.load(Ordering::Relaxed),
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };
    WORKER.fetch_add(1, Ordering::Relaxed);
    // println!("{:?}", data);
    cfg.app_data(Data::new(data));
}
