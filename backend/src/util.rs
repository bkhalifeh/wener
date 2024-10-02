use std::sync::Arc;

use axum::{extract::State, Router};
use bb8::{Pool, PooledConnection};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use crate::modules::hash::HashService;

#[derive(Clone)]
pub struct AppState<'a> {
    pub db_pool: Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
    pub hash_service: HashService<'a>,
}

pub type TAppState<'a> = State<Arc<AppState<'a>>>;
pub type TAppRouter<'a> = Router<Arc<AppState<'a>>>;
pub type TConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;
