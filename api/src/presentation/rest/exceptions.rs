use crate::domain::code::{
    CodeEntity, CodeError, HEADER_ERROR, PARSE_JSON_ERROR, PATH_ERROR, QUERY_ERROR,
};

use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum_extra::typed_header::TypedHeaderRejection;

impl CodeError for TypedHeaderRejection {
    fn code(&self) -> CodeEntity {
        HEADER_ERROR
    }
}

impl CodeError for JsonRejection {
    fn code(&self) -> CodeEntity {
        PARSE_JSON_ERROR
    }
}

impl CodeError for QueryRejection {
    fn code(&self) -> CodeEntity {
        QUERY_ERROR
    }
}

impl CodeError for PathRejection {
    fn code(&self) -> CodeEntity {
        PATH_ERROR
    }
}
