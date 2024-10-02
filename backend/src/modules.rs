use crate::util::TAppRouter;
use axum::Router;

pub mod hash;
pub mod user;

pub fn get_all_routes() -> TAppRouter<'static> {
    return Router::new().merge(user::get_user_routes());
}
