use std::ops::Deref;
use std::sync::{Arc, Mutex};
use diesel::PgConnection;
use warp;
use tokio::task::spawn_blocking;
use crate::db;
use warp::{Filter, filters, Reply};
use warp::filters::BoxedFilter;

// pub fn build_routes(conns: Arc<Mutex<PgConnection>>) -> BoxedFilter<(String, )>{
//
//
// }
