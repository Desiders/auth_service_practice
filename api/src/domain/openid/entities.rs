use super::UserInfoError;

use openidconnect::{
    core::{
        CoreClient, CoreGenderClaim, CoreJsonWebKey, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm,
    },
    EmptyAdditionalClaims, EndpointMaybeSet, EndpointNotSet, EndpointSet, UserInfoJsonWebToken,
    UserInfoVerifier,
};
use serde_json::Value;

pub type OpenIdClient = CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

#[derive(Debug)]
pub struct UserInfoClaims(
    pub openidconnect::UserInfoClaims<EmptyAdditionalClaims, CoreGenderClaim>,
);

impl UserInfoClaims {
    #[allow(clippy::missing_errors_doc)]
    pub fn from_raw_jwt(
        jwt_token: impl Into<Vec<u8>>,
        verifier: &UserInfoVerifier<CoreJweContentEncryptionAlgorithm, CoreJsonWebKey>,
    ) -> Result<Self, UserInfoError> {
        let jwt_token = String::from_utf8(jwt_token.into())?;
        let user_info_jwt = serde_path_to_error::deserialize(Value::String(jwt_token))?;
        let user_info = UserInfoJsonWebToken::<
            EmptyAdditionalClaims,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
        >::claims(user_info_jwt, verifier)?;

        Ok(Self(user_info))
    }
}
