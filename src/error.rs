use thiserror::Error;

#[derive(Debug, Error)]
pub enum GDSIIErrorKind {
    #[error("Cannot parse the given *.gds")]
    InvalidGDSII,
    #[error("Cannot open the file at `{0}`")]
    InvalidPath(String),
}
