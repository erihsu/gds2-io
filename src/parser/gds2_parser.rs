use super::variant_parser::*;
use crate::error::GDSIIErrorKind;
use crate::model::*;
use nom::multi::many1;
use nom::Finish;

pub(super) fn gds2_parser(s: &str) -> std::result::Result<GDSIIModel, GDSIIErrorKind> {
    match many1(variant_parser)(s).finish() {
        Ok((_, data)) => {
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
                        gds2_model.summerize();
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
