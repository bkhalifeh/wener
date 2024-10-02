use crate::util::TAppRouter;
use axum::Router;

pub mod hash;
pub mod user;

pub fn get_all_routes() -> TAppRouter<'static> {
    let router: TAppRouter<'static> = Router::new();
    return router;
}
