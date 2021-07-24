use crate::error::GDSIIErrorKind;
use crate::model::*;
use nom::{
    error::{ErrorKind, ParseError},
    multi::many_till,
    Finish,
};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub(self) enum ParseGDSIIError<I> {
    ParseIntError,
    Nom(I, ErrorKind),
    Utf8Error,
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
mod variant_parser;

use basic::end_tag;
use variant_parser::*;

pub fn gds2_parser(s: &[u8]) -> std::result::Result<GDSIIModel, GDSIIErrorKind> {
    match many_till(variant_parser, end_tag)(s).finish() {
        Ok((_, (data, _))) => {
            let mut gds2_model = GDSIIModel::default();
            let mut current_tuctosin_type = TuctosinHeader::default();
            // main process
            for d in data {
                match d {
                    GDSIIVariant::FileHeader(header) => {
                        gds2_model.header.insert(header.get_tag(), header);
                    }
                    GDSIIVariant::ModuleHeader(module) => match module {
                        ModuleHeader::BgnStr(t) => gds2_model.structure_time = t,
                        ModuleHeader::StrName(t) => gds2_model.structure_name = t,
                    },
                    GDSIIVariant::TuctosinHeader(toc_header) => {
                        if toc_header != current_tuctosin_type {
                            current_tuctosin_type = toc_header;
                        }
                    }
                    GDSIIVariant::Tuctosin(shape) => match current_tuctosin_type {
                        TuctosinHeader::Boundary => gds2_model.s_boundary.push(shape),
                        TuctosinHeader::Path => gds2_model.s_path.push(shape),
                        TuctosinHeader::Sref => gds2_model.s_sref.push(shape),
                        TuctosinHeader::Aref => gds2_model.s_aref.push(shape),
                        TuctosinHeader::Text => gds2_model.s_text.push(shape),
                        TuctosinHeader::Node => gds2_model.s_node.push(shape),
                        TuctosinHeader::Box => gds2_model.s_box.push(shape),
                    },
                    GDSIIVariant::FileEnd => {
                        // summerize
                        // gds2_model.summerize();
                        break;
                        // early return
                    }
                    _ => {}
                }
            }
            Ok(gds2_model)
        }
        Err(_) => Err(GDSIIErrorKind::InvalidGDSII),
    }
}
