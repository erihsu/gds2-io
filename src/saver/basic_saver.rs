use crate::model::Tuctosin;
use byteorder::{BigEndian, ByteOrder};

impl Tuctosin {
    pub fn byted(&self) -> Vec<u8> {
        let mut data = vec![];
        match &self {
            Tuctosin::ElfFlags(d) => {
                data.extend_from_slice(&[0x26, 0x01]);
                BigEndian::write_i16(&mut data, *d);
            }
            Tuctosin::Plex(d) => {
                data.extend_from_slice(&[0x2F, 0x03]);
                BigEndian::write_i32(&mut data, *d);
            }
            Tuctosin::Layer(d) => {
                data.extend_from_slice(&[0x0D, 0x02]);
                BigEndian::write_i16(&mut data, *d);
            }
            Tuctosin::DataType(d) => {
                data.extend_from_slice(&[0x0E, 0x02]);
                BigEndian::write_i16(&mut data, *d);
            }
            Tuctosin::Xy(d) => {
                data.extend_from_slice(&[0x10, 0x03]);
                for i in d {
                    // write x
                    BigEndian::write_i32(&mut data, i.0);
                    // write y
                    BigEndian::write_i32(&mut data, i.1);
                }
            }
            Tuctosin::PathType(d) => {
                data.extend_from_slice(&[0x21, 0x02]);
                BigEndian::write_i16(&mut data, *d);
            }
            Tuctosin::Width(d) => {
                data.extend_from_slice(&[0x0F, 0x03]);
                BigEndian::write_i32(&mut data, *d);
            }
            Tuctosin::Sname(d) => {
                data.extend_from_slice(&[0x12, 0x06]);
                data.extend_from_slice(d.as_bytes());
            }
            Tuctosin::Strans(d) => {
                data.extend_from_slice(&[0x1A, 0x01]);
                BigEndian::write_i16(&mut data, *d);
            }

            Tuctosin::Mag(d) => {
                data.extend_from_slice(&[0x1B, 0x05]);
                BigEndian::write_i64(&mut data, *d);
            }
            Tuctosin::Angle(d) => {
                data.extend_from_slice(&[0x1C, 0x05]);
                BigEndian::write_i64(&mut data, *d);
            }
            Tuctosin::ColRow(d) => {
                data.extend_from_slice(&[0x13, 0x02]);
                BigEndian::write_i16(&mut data, d.0);
                BigEndian::write_i16(&mut data, d.1);
            }
            Tuctosin::TextType(d) => {
                data.extend_from_slice(&[0x16, 0x02]);
                BigEndian::write_i16(&mut data, *d);
            }

            Tuctosin::Persentation(d) => {
                data.extend_from_slice(&[0x17, 0x01]);
                BigEndian::write_i16(&mut data, *d);
            }
            Tuctosin::AsciiString(d) => {
                data.extend_from_slice(&[0x19, 0x06]);
                data.extend_from_slice(d.as_bytes());
            }
            Tuctosin::NodeType(d) => {
                data.extend_from_slice(&[0x2A, 0x02]);
                BigEndian::write_i16(&mut data, *d);
            }
            Tuctosin::BoxType(d) => {
                data.extend_from_slice(&[0x2E, 0x02]);
                BigEndian::write_i16(&mut data, *d);
            }
        }
        return data;
    }
}
