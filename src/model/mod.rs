use std::collections::HashMap;

#[derive(Default)]
pub struct GDSIIModel {
    pub header: HashMap<String, FileHeader>,
    pub structure_time: [i16; 12],
    pub structure_name: String,
    pub s_boundary: Vec<Tuctosin>,
    pub s_path: Vec<Tuctosin>,
    pub s_sref: Vec<Tuctosin>,
    pub s_aref: Vec<Tuctosin>,
    pub s_text: Vec<Tuctosin>,
    pub s_node: Vec<Tuctosin>,
    pub s_box: Vec<Tuctosin>,
}

impl GDSIIModel {
    pub fn summerize(&self) {
        unimplemented!()
    }
}

pub enum GDSIIVariant {
    FileHeader(FileHeader),
    ModuleHeader(ModuleHeader),
    TuctosinHeader(TuctosinHeader),
    Tuctosin(Tuctosin),
    TuctosinEnd,
    ModuleEnd,
    FileEnd,
}

/// File header variant in GDSII
pub enum FileHeader {
    Header(i16),       // 0x00_02
    BgnLib([i16; 12]), // 0x01_02
    LibName(String),   // 0x02_06
    RefLibs(String),   // 0x1F_06
    Fonts(String),     // 0x20_06
    AttrTable(String), // 0x23_06
    Generations(i16),  // 0x22_02
    Format(i16),       // 0x36_02
    Mask(String),      // 0x37_06
    EndMask,           // 0x38_00
    Units([f64; 2]),   // 0x03_05
}

impl FileHeader {
    pub fn get_tag(&self) -> String {
        let tag_str = match self {
            FileHeader::Header(_) => "head",
            FileHeader::BgnLib(_) => "bgn",
            FileHeader::LibName(_) => "libname",
            FileHeader::RefLibs(_) => "reflib",
            FileHeader::Fonts(_) => "font",
            FileHeader::AttrTable(_) => "attr",
            FileHeader::Generations(_) => "generation",
            FileHeader::Format(_) => "format",
            FileHeader::Mask(_) => "mask",
            FileHeader::EndMask => "endmask",
            FileHeader::Units(_) => "unit",
        };
        tag_str.to_string()
    }
}

/// Module header in GDSII
pub enum ModuleHeader {
    BgnStr([i16; 12]), // 0x05_02
    StrName(String),   // 0x06_06
}

/// shape header in GDSII
#[derive(PartialEq)]
pub enum TuctosinHeader {
    Boundary, // 0x08_00
    Path,     // 0x09_00
    Sref,     // 0x0A_00
    Aref,     // 0x0B_00
    Text,     // 0x0C_00
    Node,     // 0x15_00
    Box,      // 0x2D_00
}

impl Default for TuctosinHeader {
    fn default() -> Self {
        TuctosinHeader::Boundary
    }
}

pub enum Tuctosin {
    ElfFlags(i16),       // 0x26_01
    Plex(i32),           // 0x2F_03
    Layer(i16),          // 0x0D_O2
    DataType(i16),       // 0xOE_O2
    Xy(Vec<(i32, i32)>), // 0x10_03
    PathType(i16),       // 0x21_02
    Width(i32),          // 0x0F_03
    Sname(String),       // 0x12_06
    Strans(i16),         // 0x1A_01
    Mag(i64),            // 0x1B_05
    Angle(i64),          // 0x1C_05
    ColRow((i16, i16)),  // 0x13_02
    TextType(i16),       // 0x16_02
    Persentation(i16),   // 0x17_01
    AsciiString(String), // 0x19_06
    NodeType(i16),       // 0x2A_02
    BoxType(i16),        // 0x2E_02
}
