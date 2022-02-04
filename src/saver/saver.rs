use crate::model::{FileHeader, GDSIIModel};
use crate::GDSIIErrorKind;
use byteorder::{BigEndian, ByteOrder};

impl GDSIIModel {
    /// gds2 file saver
    pub fn save_gds2<P: AsRef<std::path::Path>>(
        self,
        file: P,
    ) -> std::result::Result<(), GDSIIErrorKind> {
        let mut data: Vec<u8> = vec![];

        // save file header
        for (k, v) in self.header.iter() {
            match k.as_str() {
                "head" => {
                    if let FileHeader::Header(d) = v {
                        let mut hdr = vec![];
                        BigEndian::write_i16(&mut hdr, *d);
                        data.extend_from_slice(&[0x00, 0x02]);
                        data.extend_from_slice(&hdr);
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "bgn" => {
                    if let FileHeader::BgnLib(d) = v {
                        let mut hdr = vec![];
                        for i in d {
                            BigEndian::write_i16(&mut hdr, *i);
                        }

                        data.extend_from_slice(&[0x01, 0x02]);
                        data.extend_from_slice(&hdr);
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "libname" => {
                    if let FileHeader::LibName(d) = v {
                        data.extend_from_slice(&[0x02, 0x06]);
                        data.extend_from_slice(d.as_bytes());
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "reflib" => {
                    if let FileHeader::RefLibs(d) = v {
                        data.extend_from_slice(&[0x1F, 0x06]);
                        data.extend_from_slice(d.as_bytes());
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "font" => {
                    if let FileHeader::Fonts(d) = v {
                        data.extend_from_slice(&[0x20, 0x06]);
                        data.extend_from_slice(d.as_bytes());
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "attr" => {
                    if let FileHeader::AttrTable(d) = v {
                        data.extend_from_slice(&[0x23, 0x06]);
                        data.extend_from_slice(d.as_bytes());
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "generation" => {
                    if let FileHeader::Generations(d) = v {
                        let mut hdr = vec![];
                        BigEndian::write_i16(&mut hdr, *d);
                        data.extend_from_slice(&[0x22, 0x02]);
                        data.extend_from_slice(&hdr);
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "format" => {
                    if let FileHeader::Generations(d) = v {
                        let mut hdr = vec![];
                        BigEndian::write_i16(&mut hdr, *d);
                        data.extend_from_slice(&[0x36, 0x02]);
                        data.extend_from_slice(&hdr);
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "mask" => {
                    if let FileHeader::Mask(d) = v {
                        data.extend_from_slice(&[0x37, 0x06]);
                        data.extend_from_slice(d.as_bytes());
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "endmask" => {
                    if let FileHeader::EndMask = v {
                        data.extend_from_slice(&[0x22, 0x02]);
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                "unit" => {
                    if let FileHeader::Units(d) = v {
                        let mut hdr = vec![];
                        BigEndian::write_f64(&mut hdr, d[0]);
                        BigEndian::write_f64(&mut hdr, d[1]);
                        data.extend_from_slice(&[0x03, 0x05]);
                        data.extend_from_slice(&hdr);
                    } else {
                        panic!("bad gds2 file");
                    }
                }
                _ => {}
            }
        }

        // save tuctosin
        if self.s_boundary.len() != 0 {
            // tuctosin header
            data.extend_from_slice(&[0x08, 0x00]);
            for toc in self.s_boundary {
                data.extend_from_slice(&toc.byted());
            }
            // tuctosin end
            data.extend_from_slice(&[0x11, 0x00]);
        }
        if self.s_path.len() != 0 {
            // tuctosin header
            data.extend_from_slice(&[0x09, 0x00]);
            for toc in self.s_path {
                data.extend_from_slice(&toc.byted());
            }
            // tuctosin end
            data.extend_from_slice(&[0x11, 0x00]);
        }
        if self.s_sref.len() != 0 {
            // tuctosin header
            data.extend_from_slice(&[0x0A, 0x00]);
            for toc in self.s_sref {
                data.extend_from_slice(&toc.byted());
            }
            // tuctosin end
            data.extend_from_slice(&[0x11, 0x00]);
        }
        if self.s_aref.len() != 0 {
            data.extend_from_slice(&[0x0B, 0x00]);
            for toc in self.s_aref {
                data.extend_from_slice(&toc.byted());
            }
            // tuctosin end
            data.extend_from_slice(&[0x11, 0x00]);
        }
        if self.s_text.len() != 0 {
            data.extend_from_slice(&[0x0C, 0x00]);
            for toc in self.s_text {
                data.extend_from_slice(&toc.byted());
            }
            // tuctosin end
            data.extend_from_slice(&[0x11, 0x00]);
        }
        if self.s_node.len() != 0 {
            data.extend_from_slice(&[0x15, 0x00]);
            for toc in self.s_node {
                data.extend_from_slice(&toc.byted());
            }
            // tuctosin end
            data.extend_from_slice(&[0x11, 0x00]);
        }
        if self.s_box.len() != 0 {
            data.extend_from_slice(&[0x2D, 0x00]);
            for toc in self.s_box {
                data.extend_from_slice(&toc.byted());
            }
            // tuctosin end
            data.extend_from_slice(&[0x11, 0x00]);
        }

        // module end
        data.extend_from_slice(&[0x07, 0x00]);

        // file end
        data.extend_from_slice(&[0x04, 0x00]);

        std::fs::write(file, data)?;

        Ok(())
    }
}
