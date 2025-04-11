use crate::presentation::rest::{responses::JsonOk, OkResponse, OkResponseInner};

use axum::{http::StatusCode, response::IntoResponse};
use tracing::instrument;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(ping))]
pub struct Doc;

#[instrument(skip_all)]
#[utoipa::path(get, path = "ping", responses(
    (status = StatusCode::OK, body = OkResponseInner<&str>)
))]
pub async fn ping() -> impl IntoResponse {
    (StatusCode::OK, JsonOk(OkResponse("Pong")))
}
