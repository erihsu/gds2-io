use super::basic::*;
use super::ParseGDIIRes;
use crate::model::*;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use nom::bytes::streaming::take;

use std::io::Cursor;
pub(super) fn variant_parser(s: &[u8]) -> ParseGDIIRes<&[u8], GDSIIVariant> {
    let (s, size) = take_size(s)?;
    let (s, d_type) = take_type(s)?;
    // data size

    // FIXME there is bug when size == 0
    if size < 4usize {
        return Ok((s, GDSIIVariant::Eof));
    }
    let d_size = size - 4usize;

    let (s, data) = take(d_size)(s)?;
    // println!("{}", d_size);
    let module_header = match d_type {
        // File Header
        [0x00, 0x02] => {
            assert!(d_size == 2usize, "mismatch header length in file header");
            let header = BigEndian::read_i16(&data);
            GDSIIVariant::FileHeader(FileHeader::Header(header))
        }
        [0x01, 0x02] => {
            assert!(d_size == 24usize, "mismatch data bgn length in file header");
            let mut bgn = [0i16; 12];
            BigEndian::read_i16_into(&data, &mut bgn);
            GDSIIVariant::FileHeader(FileHeader::BgnLib(bgn))
        }
        [0x02, 0x06] => {
            // let escaped_ascii = data
            //     .iter()
            //     .map(|v| ascii::escape_default(*v).next().unwrap())
            //     .collect();
            GDSIIVariant::FileHeader(FileHeader::LibName(
                String::from_utf8(data.to_vec()).unwrap(),
            ))
        }
        [0x1F, 0x06] => {
            assert!(d_size == 90usize, "mismatch reflib length in file header");
            // let escaped_ascii = data
            //     .iter()
            //     .map(|v| ascii::escape_default(*v).next().unwrap())
            //     .collect();
            GDSIIVariant::FileHeader(FileHeader::RefLibs(
                String::from_utf8(data.to_vec()).unwrap(),
            ))
        }
        [0x20, 0x06] => {
            assert!(d_size == 176usize, "mismatch font length in file header");
            // let escaped_ascii = data
            //     .iter()
            //     .map(|v| ascii::escape_default(*v).next().unwrap())
            //     .collect();
            GDSIIVariant::FileHeader(FileHeader::Fonts(String::from_utf8(data.to_vec()).unwrap()))
        }
        [0x23, 0x06] => {
            assert!(
                d_size == 44usize,
                "mismatch attr_table length in file header"
            );
            GDSIIVariant::FileHeader(FileHeader::AttrTable(
                String::from_utf8_lossy(data).to_string(),
            ))
        }
        [0x22, 0x02] => {
            assert!(
                d_size == 2usize,
                "mismatch generation length in file header"
            );
            let generation = BigEndian::read_i16(&data);
            GDSIIVariant::FileHeader(FileHeader::Generations(generation))
        }
        [0x36, 0x02] => {
            assert!(d_size == 2usize, "mismatch format length in file header");
            let format = BigEndian::read_i16(&data);
            GDSIIVariant::FileHeader(FileHeader::Format(format))
        }
        [0x37, 0x06] => {
            GDSIIVariant::FileHeader(FileHeader::Mask(String::from_utf8_lossy(data).to_string()))
        }
        [0x38, 0x00] => {
            assert!(d_size == 0usize, "mismatch mask length in file header");
            GDSIIVariant::FileHeader(FileHeader::EndMask)
        }
        [0x03, 0x05] => {
            assert!(d_size == 16usize, "mismatch units length in file header");
            let mut units = [0.0f64; 2];
            BigEndian::read_f64_into(&data, &mut units);
            GDSIIVariant::FileHeader(FileHeader::Units(units))
        }
        // File End
        [0x04, 0x00] => {
            assert!(d_size == 0usize, "there should be no data in file tile");
            GDSIIVariant::FileEnd
        }
        // Module Header
        [0x05, 0x02] => {
            assert!(
                d_size == 24usize,
                "mismatch bgn_str length in module header"
            );
            let mut bgn = [0i16; 12];
            BigEndian::read_i16_into(&data, &mut bgn);
            GDSIIVariant::ModuleHeader(ModuleHeader::BgnStr(bgn))
        }
        [0x06, 0x06] => {
            // let escaped_ascii = data
            //     .iter()
            //     .map(|v| ascii::escape_default(*v).next().unwrap())
            //     .collect();
            GDSIIVariant::ModuleHeader(ModuleHeader::StrName(
                String::from_utf8(data.to_vec()).unwrap(),
            ))
        }
        // Module End
        [0x07, 0x00] => {
            assert!(d_size == 0usize, "there should be no data in module tile");
            GDSIIVariant::ModuleEnd
        }
        // Tuctosin header
        [0x08, 0x00] => {
            assert!(
                d_size == 0usize,
                "there should be no data in tuctosin header"
            );
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Boundary)
        }
        [0x09, 0x00] => {
            assert!(
                d_size == 0usize,
                "there should be no data in tuctosin header"
            );
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Path)
        }
        [0x0A, 0x00] => {
            assert!(
                d_size == 0usize,
                "there should be no data in tuctosin header"
            );
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Sref)
        }
        [0x0B, 0x00] => {
            assert!(
                d_size == 0usize,
                "there should be no data in tuctosin header"
            );
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Aref)
        }
        [0x0C, 0x00] => {
            assert!(
                d_size == 0usize,
                "there should be no data in tuctosin header"
            );
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Text)
        }
        [0x15, 0x00] => {
            assert!(
                d_size == 0usize,
                "there should be no data in tuctosin header"
            );
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Node)
        }
        [0x2D, 0x00] => {
            assert!(
                d_size == 0usize,
                "there should be no data in tuctosin header"
            );
            GDSIIVariant::TuctosinHeader(TuctosinHeader::Box)
        }
        // Tuctosin End
        [0x11, 0x00] => {
            assert!(d_size == 0usize, "there should be no data in tuctosin tile");
            GDSIIVariant::TuctosinEnd
        }
        // Tuctosin Body
        [0x26, 0x01] => {
            assert!(
                d_size == 2usize,
                "mismatch elf_flags length in tuctosin body"
            );
            let elfflags = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::ElfFlags(elfflags))
        }
        [0x2F, 0x03] => {
            assert!(d_size == 4usize, "mismatch plex length in tuctosin body");
            // let byted = data.as_bytes();
            let plex = BigEndian::read_i32(&data);
            GDSIIVariant::Tuctosin(Tuctosin::Plex(plex))
        }
        [0x0D, 0x02] => {
            assert!(d_size == 2usize, "mismatch layer length in tuctosin body");
            let layer = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::Layer(layer))
        }
        [0x0E, 0x02] => {
            assert!(
                d_size == 2usize,
                "mismatch data_type length in tuctosin body"
            );
            let data_type = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::DataType(data_type))
        }
        // tuctosin body
        // TODO: seek better solution
        [0x10, 0x03] => {
            assert!(d_size % 8 == 0, "mismatch x_y length in tuctosin body");
            let mut reader = Cursor::new(data);
            let mut shapes = vec![];
            let shape_len = d_size / 8;
            for _ in 0..shape_len {
                let x = reader.read_i32::<BigEndian>().unwrap();
                let y = reader.read_i32::<BigEndian>().unwrap();
                shapes.push((x, y));
            }
            GDSIIVariant::Tuctosin(Tuctosin::Xy(shapes))
        }
        [0x21, 0x02] => {
            assert!(
                d_size == 2usize,
                "mismatch path_type length in tuctosin body"
            );
            let path_type = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::PathType(path_type))
        }
        [0x0F, 0x03] => {
            assert!(d_size == 4usize, "mismatch width length in tuctosin body");
            let width = BigEndian::read_i32(&data);
            GDSIIVariant::Tuctosin(Tuctosin::Width(width))
        }
        [0x12, 0x06] => {
            GDSIIVariant::Tuctosin(Tuctosin::Sname(String::from_utf8_lossy(data).to_string()))
        }
        [0x1A, 0x01] => {
            assert!(
                d_size == 2usize,
                "mismatch path_type length in tuctosin body"
            );
            let strans = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::PathType(strans))
        }
        [0x1B, 0x05] => {
            assert!(d_size == 8usize, "mismatch mag length in tuctosin body");
            let mag = BigEndian::read_i64(&data);
            GDSIIVariant::Tuctosin(Tuctosin::Mag(mag))
        }
        [0x1C, 0x05] => {
            assert!(d_size == 8usize, "mismatch angle length in tuctosin body");
            let angle = BigEndian::read_i64(&data);
            GDSIIVariant::Tuctosin(Tuctosin::Angle(angle))
        }
        [0x13, 0x02] => {
            assert!(d_size == 4usize, "mismatch col_row length in tuctosin body");
            // let byted = data.as_bytes();
            let col = BigEndian::read_i16(&data);
            let row = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::ColRow((col, row)))
        }
        [0x16, 0x02] => {
            assert!(
                d_size == 2usize,
                "mismatch text_type length in tuctosin body"
            );
            let text_type = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::TextType(text_type))
        }
        [0x17, 0x01] => {
            assert!(
                d_size == 2usize,
                "mismatch persentation length in tuctosin body"
            );
            let pers = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::Persentation(pers))
        }
        [0x19, 0x06] => GDSIIVariant::Tuctosin(Tuctosin::AsciiString(
            String::from_utf8_lossy(data).to_string(),
        )),
        [0x2A, 0x02] => {
            assert!(
                d_size == 2usize,
                "mismatch node_type length in tuctosin body"
            );
            let node_type = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::NodeType(node_type))
        }
        [0x2E, 0x02] => {
            assert!(
                d_size == 2usize,
                "mismatch box_type length in tuctosin body"
            );
            let box_type = BigEndian::read_i16(&data);
            GDSIIVariant::Tuctosin(Tuctosin::BoxType(box_type))
        }
        _ => unreachable!(),
    };
    Ok((s, module_header))
}
