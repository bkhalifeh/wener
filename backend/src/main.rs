use std::env;
use std::error::Error;
use std::sync::Arc;

use modules::hash::HashService;
use tokio::net::TcpListener;

use axum::Router;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use bb8::Pool;

use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use util::AppState;

pub mod modules;
pub mod schema;
pub mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let host: String = env::var("APP_HOST")?;
    let port: u16 = env::var("APP_PORT")?.parse()?;
    let database_url: String = env::var("DATABASE_URL")?;
    let argon_secret: Vec<u8> = BASE64_STANDARD.decode(env::var("ARGON_SECRET")?)?;
    let argon_secret: &mut [u8] = Box::leak(argon_secret.into_boxed_slice());

    let db_connection_manager: AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let db_pool: Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>> =
        bb8::Pool::builder().build(db_connection_manager).await?;

    let app_state: AppState<'_> = AppState {
        db_pool,
        hash_service: HashService::new(argon_secret),
    };

    let app: Router = Router::new()
        .merge(modules::get_all_routes())
        .with_state(Arc::new(app_state));
    let listener: TcpListener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    return Ok(());
}
