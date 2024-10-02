use crate::util::TAppRouter;
use axum::Router;

pub mod auth;

pub fn get_user_routes() -> TAppRouter<'static> {
    return Router::new().nest("/user", auth::get_auth_routes());
}
