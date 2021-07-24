use super::basic::*;
use super::ParseGDIIRes;
use crate::model::*;
use byteorder::{ByteOrder, LittleEndian};
use nom::bytes::streaming::take;

pub(super) fn variant_parser(s: &str) -> ParseGDIIRes<&str, GDSIIVariant> {
    let (s, d_size) = take_size(s)?;
    let (s, d_type) = take_type(s)?;
    let (s, data) = take(d_size)(s)?;
    let module_header = match d_type.as_bytes() {
        // File Header
        b"0x0002" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let header = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::FileHeader(FileHeader::Header(header))
        }
        b"0x0102" => {
            assert!(d_size == 24, "data type mismatch in GDSII header");
            let mut bgn = [0i16; 12];
            let byted = data.as_bytes();
            for i in 0..12 {
                bgn[i] = LittleEndian::read_i16(&byted)
            }
            GDSIIVariant::FileHeader(FileHeader::BgnLib(bgn))
        }
        b"0x0206" => GDSIIVariant::FileHeader(FileHeader::LibName(data.to_string())),
        b"0x1F06" => {
            assert!(d_size == 90, "data type mismatch in GDSII header");
            GDSIIVariant::FileHeader(FileHeader::RefLibs(data.to_string()))
        }
        b"0x2006" => {
            assert!(d_size == 176, "data type mismatch in GDSII header");
            GDSIIVariant::FileHeader(FileHeader::Fonts(data.to_string()))
        }
        b"0x2306" => {
            assert!(d_size == 44, "data type mismatch in GDSII header");
            GDSIIVariant::FileHeader(FileHeader::AttrTable(data.to_string()))
        }
        b"0x2202" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            GDSIIVariant::FileHeader(FileHeader::Generations(
                i16::from_str_radix(data, 16).unwrap(),
            ))
        }
        b"0x3602" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            GDSIIVariant::FileHeader(FileHeader::Format(i16::from_str_radix(data, 16).unwrap()))
        }
        b"0x3706" => GDSIIVariant::FileHeader(FileHeader::Mask(data.to_string())),
        b"0x3800" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::FileHeader(FileHeader::EndMask)
        }
        b"0x0305" => {
            assert!(d_size == 16, "data type mismatch in GDSII header");
            let mut units = [0.0f64; 2];
            let byted = data.as_bytes();
            for i in 0..2 {
                units[i] = LittleEndian::read_f64(&byted)
            }
            GDSIIVariant::FileHeader(FileHeader::Units(units))
        }
        // File End
        b"0x0400" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::FileEnd
        }
        // Module Header
        b"0x0502" => {
            assert!(d_size == 24, "data type mismatch in GDSII header");
            let mut bgn = [0i16; 12];
            let byted = data.as_bytes();
            for i in 0..12 {
                bgn[i] = LittleEndian::read_i16(&byted)
            }
            GDSIIVariant::ModuleHeader(ModuleHeader::BgnStr(bgn))
        }
        b"0x0606" => {
            assert!(d_size == 176, "data type mismatch in GDSII header");
            GDSIIVariant::ModuleHeader(ModuleHeader::StrName(data.to_string()))
        }
        // Module End
        b"0x0700" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::ModuleEnd
        }
        // Tuctosin header
        b"0x0800" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Boundary)
        }
        b"0x0900" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Path)
        }
        b"0x0A00" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Sref)
        }
        b"0x0B00" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Aref)
        }
        b"0x0C00" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Text)
        }
        b"0x1500" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Node)
        }
        b"0x2D00" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Box)
        }
        // Tuctosin End
        b"0x1100" => {
            assert!(d_size == 0, "data type mismatch in GDSII header");
            GDSIIVariant::TuctosinEnd
        }
        // Tuctosin Body
        b"0x26_01" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let elfflags = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::ElfFlags(elfflags))
        }
        b"0x2F_03" => {
            assert!(d_size == 4, "data type mismatch in GDSII header");
            let byted = data.as_bytes();
            let plex = LittleEndian::read_i32(&byted);
            GDSIIVariant::Tuctosin(Tuctosin::Plex(plex))
        }
        b"0x0D_O2" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let layer = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::Layer(layer))
        }
        b"0xOE_O2" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let data_type = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::DataType(data_type))
        }
        b"0x10_03" => {
            assert!(d_size % 8 == 0, "data type mismatch in GDSII header");
            let mut shapes = vec![];
            let shape_len = d_size / 8;
            let byted = data.as_bytes();
            for _ in 0..shape_len {
                let x = LittleEndian::read_i32(&byted);
                let y = LittleEndian::read_i32(&byted);
                shapes.push((x, y));
            }
            GDSIIVariant::Tuctosin(Tuctosin::Xy(shapes))
        }
        b"0x21_02" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let path_type = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::PathType(path_type))
        }
        b"0x0F_03" => {
            assert!(d_size == 4, "data type mismatch in GDSII header");
            let width = i32::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::Width(width))
        }
        b"0x12_06" => GDSIIVariant::Tuctosin(Tuctosin::Sname(data.to_string())),
        b"0x1A_01" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let strans = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::PathType(strans))
        }
        b"0x1B_05" => {
            assert!(d_size == 8, "data type mismatch in GDSII header");
            let mag = i64::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::Mag(mag))
        }
        b"0x1C_05" => {
            assert!(d_size == 8, "data type mismatch in GDSII header");
            let angle = i64::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::Angle(angle))
        }
        b"0x13_02" => {
            assert!(d_size == 4, "data type mismatch in GDSII header");
            let byted = data.as_bytes();
            let col = LittleEndian::read_i16(&byted);
            let row = LittleEndian::read_i16(&byted);
            GDSIIVariant::Tuctosin(Tuctosin::ColRow((col, row)))
        }
        b"0x16_02" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let text_type = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::TextType(text_type))
        }
        b"0x17_01" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let pers = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::Persentation(pers))
        }
        b"0x19_06" => GDSIIVariant::Tuctosin(Tuctosin::AsciiString(data.to_string())),
        b"0x2A_02" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let node_type = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::NodeType(node_type))
        }
        b"0x2E_02" => {
            assert!(d_size == 2, "data type mismatch in GDSII header");
            let box_type = i16::from_str_radix(data, 16).unwrap();
            GDSIIVariant::Tuctosin(Tuctosin::BoxType(box_type))
        }
        _ => unreachable!(),
    };
    Ok((s, module_header))
}
