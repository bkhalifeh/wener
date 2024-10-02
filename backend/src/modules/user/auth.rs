use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::{
    schema::users,
    util::{internal_server_error, not_acceptable, TAppRouter, TAppState, TConnection, TResponse},
};

#[derive(Deserialize, Debug, Insertable, ToSchema)]
#[diesel(table_name=crate::schema::users)]
pub struct UserSignUpDto {
    email: String,
    password: String,
}

pub fn get_auth_routes() -> TAppRouter<'static> {
    return Router::new().route("/auth/sign-up", post(sign_up_post));
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: i32,
    email: String,
    profile: String,
}

#[utoipa::path(
    post,
    path = "/auth/sign-up",
    responses(
        (status = 201, body = [UserSignUpDto])
    ),
    tag = "user",
)]
pub async fn sign_up_post<'a>(
    State(state): TAppState<'a>,
    Json(mut body): Json<UserSignUpDto>,
) -> TResponse<Json<User>, String> {
    let mut conn: TConnection = state.db_pool.get().await.map_err(internal_server_error)?;
    body.password = state.hash_service.hash(&body.password);
    let res = diesel::insert_into(users::table)
        .values(body)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(not_acceptable)?;
    return Ok((StatusCode::CREATED, Json(res)));
}
