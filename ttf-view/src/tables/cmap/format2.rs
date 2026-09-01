use crate::types::{int16, uint16};

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat2Repr {
    pub sub_header_keys: [uint16; 256],
    sub_headers: [SubHeaderRepr; 0],
    glyph_id_array: [uint16; 0],
}

#[repr(C)]
#[non_exhaustive]
pub struct SubHeaderRepr {
    pub first_code: uint16,
    pub entry_count: uint16,
    pub id_delta: int16,
    pub id_range_offset: uint16,
}

// TODO: 'cmap' subtable format 2
