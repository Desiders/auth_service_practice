use super::CodeEntity;

use either::Either;
use std::{convert::Infallible, error::Error};

pub trait CodeError: Error {
    #[must_use]
    fn code(&self) -> CodeEntity;
}

impl CodeError for Infallible {
    fn code(&self) -> CodeEntity {
        unreachable!()
    }
}

impl<L, R> CodeError for Either<L, R>
where
    L: CodeError,
    R: CodeError,
{
    fn code(&self) -> CodeEntity {
        match self {
            Either::Left(left) => left.code(),
            Either::Right(right) => right.code(),
        }
    }
}
