use serde::Deserialize;
use std::cell::Cell;
use std::sync::{atomic::AtomicUsize, Arc};
#[derive(Clone, Deserialize, Debug)]
pub struct Worker {
    pub worker_count: usize,
    pub local_count: Cell<usize>,
    pub global_count: Arc<AtomicUsize>,
}
