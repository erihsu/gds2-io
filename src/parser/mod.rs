use nom::error::{ErrorKind, ParseError};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub(self) enum ParseGDSIIError<I> {
    ParseIntError,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for ParseGDSIIError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParseGDSIIError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

type ParseGDIIRes<T, U> = nom::IResult<T, U, ParseGDSIIError<T>>;

mod basic;
mod gds2_parser;
// mod file_tag;
mod variant_parser;
