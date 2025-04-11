pub mod root;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "API"),
    nest(
        (path = "/", api = root::Doc),
    ),
)]
pub struct ApiDoc;
