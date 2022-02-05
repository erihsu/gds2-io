use crate::model::{FileHeader, GDSIIModel};
use crate::GDSIIErrorKind;

use super::basic_saver::append_tuctosin_end;
use byteorder::{BigEndian, ByteOrder};
use std::io::BufWriter;
use std::io::Write;

impl GDSIIModel {
    /// gds2 file saver
    pub fn save_gds2<P: AsRef<std::path::Path>>(
        self,
        file: P,
    ) -> std::result::Result<(), GDSIIErrorKind> {
        let mut data: Vec<u8> = vec![];
        let mut byted_16_d = [0u8; 2];
        let mut byted_64_d = [0u8; 8];
        let mut byted_size = [0u8; 2];
        let mut file_buffer = BufWriter::new(std::fs::File::create(file)?);
        // save file header
        println!("write file header");
        if let Some(FileHeader::Header(d)) = self.header.get("head") {
            BigEndian::write_u16(&mut byted_size, 2 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x00, 0x02]);
            BigEndian::write_i16(&mut byted_16_d, *d);
            data.extend(&byted_16_d.to_vec());
        } else {
            panic!("bad gds2 file");
        }
        if let Some(FileHeader::BgnLib(d)) = self.header.get("bgn") {
            BigEndian::write_u16(&mut byted_size, 24 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x01, 0x02]);
            for i in d {
                BigEndian::write_i16(&mut byted_16_d, *i);
                data.extend(&byted_16_d.to_vec());
            }
        } else {
            panic!("bad gds2 file");
        }
        if let Some(FileHeader::LibName(d)) = self.header.get("libname") {
            BigEndian::write_u16(&mut byted_size, d.as_bytes().len() as u16 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x02, 0x06]);
            data.extend(d.as_bytes());
        } else {
            panic!("bad gds2 file");
        }
        if let Some(FileHeader::Units(d)) = self.header.get("unit") {
            BigEndian::write_u16(&mut byted_size, 16 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x03, 0x05]);
            BigEndian::write_f64(&mut byted_64_d, d[0]);
            data.extend(&byted_64_d.to_vec());
            BigEndian::write_f64(&mut byted_64_d, d[1]);
            data.extend(&byted_64_d.to_vec());
        }
        if let Some(FileHeader::RefLibs(d)) = self.header.get("reflib") {
            BigEndian::write_u16(&mut byted_size, d.as_bytes().len() as u16 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x1F, 0x06]);
            data.extend(d.as_bytes());
        }
        if let Some(FileHeader::Fonts(d)) = self.header.get("font") {
            BigEndian::write_u16(&mut byted_size, d.as_bytes().len() as u16 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x20, 0x06]);
            data.extend(d.as_bytes());
        }

        if let Some(FileHeader::AttrTable(d)) = self.header.get("attr") {
            BigEndian::write_u16(&mut byted_size, d.as_bytes().len() as u16 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x23, 0x06]);
            data.extend(d.as_bytes());
        }
        if let Some(FileHeader::Generations(d)) = self.header.get("generation") {
            BigEndian::write_u16(&mut byted_size, 2 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x22, 0x02]);
            BigEndian::write_i16(&mut byted_16_d, *d);
            data.extend(&byted_16_d.to_vec());
        }
        if let Some(FileHeader::Format(d)) = self.header.get("format") {
            BigEndian::write_u16(&mut byted_size, 2 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x36, 0x02]);
            BigEndian::write_i16(&mut byted_16_d, *d);
            data.extend(&byted_16_d.to_vec());
        }

        if let Some(FileHeader::Mask(d)) = self.header.get("mask") {
            BigEndian::write_u16(&mut byted_size, d.as_bytes().len() as u16 + 4);
            data.extend(&byted_size.to_vec());
            data.extend(&[0x37, 0x06]);
            data.extend(d.as_bytes());
        }
        file_buffer.write_all(&data)?;
        file_buffer.flush()?;
        data.clear();

        // module header
        BigEndian::write_u16(&mut byted_size, 2 * 12 + 4);
        data.extend(&byted_size);
        data.extend(&[0x05, 0x02]);
        for d in self.structure_time {
            BigEndian::write_i16(&mut byted_16_d, d);
            data.extend(byted_16_d);
        }

        BigEndian::write_u16(
            &mut byted_size,
            self.structure_name.as_bytes().len() as u16 + 4,
        );

        data.extend(&byted_size);
        data.extend(&[0x06, 0x06]);
        data.extend(self.structure_name.as_bytes());

        // tuctosin
        if self.s_boundary.len() != 0 {
            // tuctosin header
            BigEndian::write_u16(&mut byted_size, 0 + 4);
            data.extend(&byted_size);
            data.extend(&[0x08, 0x00]);
            for toc in self.s_boundary {
                data.extend(&toc.byted());
            }
            // tuctosin end
            append_tuctosin_end(&mut data);
        }
        if self.s_path.len() != 0 {
            // tuctosin header
            BigEndian::write_u16(&mut byted_size, 0 + 4);
            data.extend(&byted_size);
            data.extend(&[0x09, 0x00]);
            for toc in self.s_path {
                data.extend(&toc.byted());
            }
            // tuctosin end
            append_tuctosin_end(&mut data);
        }
        if self.s_sref.len() != 0 {
            // tuctosin header
            BigEndian::write_u16(&mut byted_size, 0 + 4);
            data.extend(&byted_size);
            data.extend(&[0x0A, 0x00]);
            for toc in self.s_sref {
                data.extend(&toc.byted());
            }
            // tuctosin end
            append_tuctosin_end(&mut data);
        }
        if self.s_aref.len() != 0 {
            BigEndian::write_u16(&mut byted_size, 0 + 4);
            data.extend(&byted_size);
            data.extend(&[0x0B, 0x00]);
            for toc in self.s_aref {
                data.extend(&toc.byted());
            }
            // tuctosin end
            append_tuctosin_end(&mut data);
        }
        if self.s_text.len() != 0 {
            BigEndian::write_u16(&mut byted_size, 0 + 4);
            data.extend(&byted_size);
            data.extend(&[0x0C, 0x00]);
            for toc in self.s_text {
                data.extend(&toc.byted());
            }
            // tuctosin end
            append_tuctosin_end(&mut data);
        }
        if self.s_node.len() != 0 {
            BigEndian::write_u16(&mut byted_size, 0 + 4);
            data.extend(&byted_size);
            data.extend(&[0x15, 0x00]);
            for toc in self.s_node {
                data.extend(&toc.byted());
            }
            // tuctosin end
            append_tuctosin_end(&mut data);
        }
        if self.s_box.len() != 0 {
            data.clear();
            BigEndian::write_u16(&mut byted_size, 0 + 4);
            data.extend(&byted_size);
            data.extend(&[0x2D, 0x00]);
            for toc in self.s_box {
                data.extend(&toc.byted());
            }
            // tuctosin end
            append_tuctosin_end(&mut data);
        }
        // if let Some(FileHeader::EndMask) = self.header.get("endmask") {
        //     BigEndian::write_i16(&mut byted_size, 0 + 4);
        //     data.extend(&byted_size);
        //     data.extend(&[0x22, 0x02]);
        // } else {
        //     panic!("bad gds2 file");
        // }
        // module end
        BigEndian::write_u16(&mut byted_size, 0 + 4);
        data.extend(&byted_size);
        data.extend(&[0x07, 0x00]);

        // file end
        BigEndian::write_u16(&mut byted_size, 0 + 4);
        data.extend(&byted_size);
        data.extend(&[0x04, 0x00]);
        file_buffer.write_all(&data)?;
        file_buffer.flush()?;
        Ok(())
    }
}
