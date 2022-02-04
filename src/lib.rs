#![allow(dead_code)]

use crate::model::GDSIIModel;
use crate::parser::gds2_parser;

use crate::error::GDSIIErrorKind;

mod error;
mod model;
mod parser;
mod saver;

/// gds2 file path
pub fn parse_gds2<P: AsRef<std::path::Path>>(
    file: P,
) -> std::result::Result<GDSIIModel, GDSIIErrorKind> {
    let buff = std::fs::read(file)?;
    let gds2: GDSIIModel = gds2_parser(&buff)?;
    Ok(gds2)
}
