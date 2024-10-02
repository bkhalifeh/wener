use axum::{
    extract::State,
    routing::{get, post},
    Form, Router,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;

use crate::util::{TAppRouter, TAppState, TConnection};

struct UserAuthSignUpTemplate {}

// #[derive(Deserialize, Debug, Insertable)]
// #[diesel(table_name=users)]
struct UserSignUpDto {
    email: String,
    password: String,
}

pub fn get_auth_routes() -> TAppRouter<'static> {
    let router: TAppRouter<'static> = Router::new();
    //.route("/auth/sign-up", post(sign_up_post));
    return router;
}

// async fn sign_up_post<'a>(State(state): TAppState<'a>, Form(input): Form<UserSignUpDto>) {
//     // let mut conn: TConnection = state.db_pool.get().await.unwrap();
//     // let res = diesel::insert_into(users::table)
//     //     .values(input)
//     //     .execute(&mut conn)
//     //     .await;
//     // match res {
//     //     Ok(c) => {
//     //         println!("{c}");
//     //     }
//     //     Err(msg) => {
//     //         println!("{msg:?}");
//     //     }
//     // }
//     // return UserAuthSignUpTemplate {};
// }
