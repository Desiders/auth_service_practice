use crate::{domain::code::CodeError, presentation::rest::JsonErr};

use axum::{
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Request},
    http::request::Parts,
    Json,
};
use serde::de::DeserializeOwned;

impl<T, S> FromRequestParts<S> for JsonErr<T>
where
    T: FromRequestParts<S>,
    T::Rejection: CodeError,
    S: Send + Sync,
{
    type Rejection = JsonErr<T::Rejection>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match T::from_request_parts(parts, state).await {
            Ok(res) => Ok(Self(res)),
            Err(err) => Err(JsonErr(err)),
        }
    }
}

impl<T, S> FromRequest<S> for JsonErr<Json<T>>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = JsonErr<JsonRejection>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(res) => Ok(Self(res)),
            Err(err) => Err(JsonErr(err)),
        }
    }
}
