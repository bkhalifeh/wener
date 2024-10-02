use crate::modules::user::auth;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "user")
    ),
    paths(auth::sign_up_post,),
    components(schemas(auth::UserSignUpDto, auth::User)),
    info(
        title = "Wener",
        description = "The Wener API description",
    ),
)]
pub struct ApiDoc;
