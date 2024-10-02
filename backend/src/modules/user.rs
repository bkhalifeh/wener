use axum::Router;

use crate::util::TAppRouter;

pub mod auth;

pub fn get_user_routes() -> TAppRouter<'static> {
    let router: TAppRouter<'static> = Router::new().nest("/user", auth::get_auth_routes());
    return router;
}
