use crate::domain::code::{
    CodeEntity, CodeError, CLAIMS_ERROR, PARSE_JSON_ERROR, UTF8_ENCODING_ERROR,
};

use openidconnect::ClaimsVerificationError;
use std::string::FromUtf8Error;

#[derive(Debug, thiserror::Error)]
pub enum UserInfoError {
    #[error(transparent)]
    ClaimsVerification(#[from] ClaimsVerificationError),
    #[error(transparent)]
    Parse(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error(transparent)]
    FromUtf8(#[from] FromUtf8Error),
}

impl CodeError for UserInfoError {
    fn code(&self) -> CodeEntity {
        match self {
            UserInfoError::ClaimsVerification(_) => CLAIMS_ERROR,
            UserInfoError::Parse(_) => PARSE_JSON_ERROR,
            UserInfoError::FromUtf8(_) => UTF8_ENCODING_ERROR,
        }
    }
}
