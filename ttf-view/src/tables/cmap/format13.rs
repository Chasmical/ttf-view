use crate::types::uint32;

#[repr(C)]
#[non_exhaustive]
pub struct Format13 {
    pub num_groups: uint32,
    groups: [ConstantMapGroup; 0],
}

#[repr(C)]
pub struct ConstantMapGroup {
    pub start_char_code: uint32,
    pub end_char_code: uint32,
    pub glyph_id: uint32,
}

impl super::CmapSubtableTrait for Format13 {
    const FORMAT: u16 = 13;
}

// TODO: 'cmap' subtable format 13
