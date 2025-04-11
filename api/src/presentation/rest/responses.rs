use crate::domain::{
    code::{CodeEntity, CodeError, UNEXPECTED_ERROR},
    common::ErrorKind,
};

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

pub struct JsonOk<T>(pub T);

impl<T> From<T> for JsonOk<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Serialize)]
pub struct OkResponse<R>(pub R);

#[derive(Serialize, ToSchema)]
pub struct OkResponseInner<R: ?Sized> {
    #[schema(example = true)]
    is_success: bool,
    result: R,
}

trait OkIntoResponse<R> {
    #[must_use]
    fn into_response(self) -> OkResponseInner<R>;
}

impl<R> OkIntoResponse<R> for R {
    fn into_response(self) -> OkResponseInner<R> {
        OkResponseInner {
            is_success: true,
            result: self,
        }
    }
}

impl<R: OkIntoResponse<R> + Serialize> IntoResponse for JsonOk<R> {
    fn into_response(self) -> Response {
        IntoResponse::into_response(Json(self.0.into_response()))
    }
}

pub struct JsonErr<T>(pub T);

impl<T> From<T> for JsonErr<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponseInner {
    #[schema(example = 1000)]
    code: u16,
    #[schema(example = false)]
    is_success: bool,
    #[schema(example = "Unexpected")]
    name: &'static str,
    #[schema(example = "...")]
    message: String,
}

trait ErrorIntoResponse {
    #[must_use]
    fn into_response(self) -> ErrorResponseInner;
}

impl<E: CodeError> ErrorIntoResponse for E {
    fn into_response(self) -> ErrorResponseInner {
        let CodeEntity { code, name } = self.code();

        error!(err = ?self, code, name, "Exception occurs");

        ErrorResponseInner {
            code,
            is_success: false,
            name,
            message: match self.source() {
                Some(inner_err) => format!("{self}: {inner_err}"),
                None => self.to_string(),
            },
        }
    }
}

impl<E: CodeError> ErrorIntoResponse for ErrorKind<E> {
    fn into_response(self) -> ErrorResponseInner {
        match self {
            Self::Expected(err) => ErrorIntoResponse::into_response(err),
            Self::Unexpected(err) => {
                let CodeEntity { code, name } = UNEXPECTED_ERROR;

                error!(?err, code, name, "Unexpected exception occurs");

                ErrorResponseInner {
                    code,
                    is_success: false,
                    name,
                    message: match err.source() {
                        Some(inner_err) => format!("{err}: {inner_err}"),
                        None => err.to_string(),
                    },
                }
            }
        }
    }
}

impl<E: ErrorIntoResponse> IntoResponse for JsonErr<E> {
    fn into_response(self) -> Response {
        IntoResponse::into_response(Json(self.0.into_response()))
    }
}
