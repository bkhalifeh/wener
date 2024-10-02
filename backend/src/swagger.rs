use crate::modules::user::auth;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "user")
    ),
    paths(auth::sign_up_post,),
    components(schemas(auth::UserSignUpDto, auth::User)),
)]
pub struct ApiDoc;
