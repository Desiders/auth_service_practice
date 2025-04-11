use crate::{
    domain::openid::{UserInfoClaimsEntity, UserInfoError},
    presentation::rest::{AppState, JsonErr},
};

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    typed_header::TypedHeaderRejection,
    TypedHeader,
};
use either::Either::{self, Left, Right};

impl FromRequestParts<AppState> for UserInfoClaimsEntity {
    type Rejection = (
        StatusCode,
        JsonErr<Either<UserInfoError, TypedHeaderRejection>>,
    );

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let authorization = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
            .await
            .map_err(|err| (StatusCode::UNAUTHORIZED, JsonErr(Right(err))))?;
        let jwt_token = authorization.0 .0.token();
        let user_info_verifier = state.identity_provider_client.user_info_verifier(None);

        Self::from_raw_jwt(jwt_token, &user_info_verifier)
            .map_err(|err| (StatusCode::UNAUTHORIZED, JsonErr(Left(err))))
    }
}
