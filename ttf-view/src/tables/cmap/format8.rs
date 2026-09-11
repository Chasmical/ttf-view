use crate::types::uint32;

#[repr(C)]
#[non_exhaustive]
pub struct Format8 {
    pub is32: [u8; 8192],
    pub num_groups: uint32,
    groups: [SequentialMapGroup; 0],
}

#[repr(C)]
pub struct SequentialMapGroup {
    pub start_char_code: uint32,
    pub end_char_code: uint32,
    pub start_glyph_id: uint32,
}

impl super::CmapSubtableTrait for Format8 {
    const FORMAT: u16 = 8;
}

// TODO: 'cmap' subtable format 8
