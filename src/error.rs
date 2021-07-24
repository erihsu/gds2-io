use thiserror::Error;

#[derive(Debug, Error)]
pub enum GDSIIErrorKind {
    #[error("Cannot parse the given *.gds")]
    InvalidGDSII,
    #[error("IO error while opening gds file")]
    InvalidPath {
        #[from]
        source: std::io::Error,
    },
}
