use crate::types::uint32;

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat13Repr {
    pub num_groups: uint32,
    groups: [ConstantMapGroupRepr; 0],
}

#[repr(C)]
pub struct ConstantMapGroupRepr {
    pub start_char_code: uint32,
    pub end_char_code: uint32,
    pub glyph_id: uint32,
}

impl super::CmapSubtableTrait for CmapSubtableFormat13Repr {
    const FORMAT: u16 = 13;
}

// TODO: 'cmap' subtable format 13
