use crate::model::Tuctosin;
use byteorder::{BigEndian, ByteOrder};

pub fn append_tuctosin_end(data: &mut Vec<u8>) {
    let mut byted_size = [0u8; 2];
    BigEndian::write_u16(&mut byted_size, 0 + 4);
    data.extend(&byted_size);
    data.extend(&[0x11, 0x00]);
}

impl Tuctosin {
    pub fn byted(&self) -> Vec<u8> {
        let mut data = vec![];
        let mut byted_size = [0; 2];
        match &self {
            Tuctosin::ElfFlags(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x26, 0x01]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::Plex(d) => {
                let mut byted_d = [0; 4];
                BigEndian::write_i16(&mut byted_size, 4 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x2F, 0x03]);
                BigEndian::write_i32(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::Layer(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x0D, 0x02]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::DataType(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x0E, 0x02]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::Xy(d) => {
                let mut byted_d = [0; 4];
                BigEndian::write_i16(&mut byted_size, (d.len() * 8) as i16 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x10, 0x03]);
                for i in d {
                    // write x
                    BigEndian::write_i32(&mut byted_d, i.0);
                    data.extend_from_slice(&byted_d.to_vec());
                    // write y
                    BigEndian::write_i32(&mut byted_d, i.1);
                    data.extend_from_slice(&byted_d.to_vec());
                }
            }
            Tuctosin::PathType(d) => {
                let mut byted_d = [0; 4];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x21, 0x02]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::Width(d) => {
                let mut byted_d = [0; 4];
                BigEndian::write_i16(&mut byted_size, 4 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x0F, 0x03]);
                BigEndian::write_i32(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::Sname(d) => {
                BigEndian::write_i16(&mut byted_size, d.as_bytes().len() as i16 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x12, 0x06]);
                data.extend_from_slice(d.as_bytes());
            }
            Tuctosin::Strans(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x1A, 0x01]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }

            Tuctosin::Mag(d) => {
                let mut byted_d = [0; 8];
                BigEndian::write_i16(&mut byted_size, 8 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x1B, 0x05]);
                BigEndian::write_i64(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::Angle(d) => {
                let mut byted_d = [0; 8];
                BigEndian::write_i16(&mut byted_size, 8 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x1C, 0x05]);
                BigEndian::write_i64(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::ColRow(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 4 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x13, 0x02]);
                BigEndian::write_i16(&mut byted_d, d.0);
                data.extend_from_slice(&byted_d.to_vec());
                BigEndian::write_i16(&mut byted_d, d.1);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::TextType(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x16, 0x02]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }

            Tuctosin::Persentation(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x17, 0x01]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::AsciiString(d) => {
                BigEndian::write_i16(&mut byted_size, d.as_bytes().len() as i16 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x19, 0x06]);
                data.extend_from_slice(d.as_bytes());
            }
            Tuctosin::NodeType(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x2A, 0x02]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
            Tuctosin::BoxType(d) => {
                let mut byted_d = [0; 2];
                BigEndian::write_i16(&mut byted_size, 2 + 4);
                data.extend_from_slice(&byted_size);
                data.extend_from_slice(&[0x2E, 0x02]);
                BigEndian::write_i16(&mut byted_d, *d);
                data.extend_from_slice(&byted_d.to_vec());
            }
        }
        return data;
    }
}
