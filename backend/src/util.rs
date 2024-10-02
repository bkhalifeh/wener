use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Router};
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use crate::modules::hash::HashService;

#[derive(Clone)]
pub struct AppState<'a> {
    pub db_pool: Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
    pub redis_pool: Pool<RedisConnectionManager>,
    pub hash_service: HashService<'a>,
}

pub type TAppState<'a> = State<Arc<AppState<'a>>>;
pub type TAppRouter<'a> = Router<Arc<AppState<'a>>>;
pub type TConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub fn internal_server_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn not_acceptable<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::NOT_ACCEPTABLE, err.to_string())
}

pub type TResponse<T, E> = Result<(StatusCode, T), (StatusCode, E)>;
