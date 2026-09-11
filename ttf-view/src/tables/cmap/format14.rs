use crate::types::{Offset32, uint16, uint24, uint32};

#[repr(C)]
#[non_exhaustive]
pub struct Format14 {
    pub num_var_selector_records: uint32,
    var_selectors: [VariationSelector; 0],
}

#[repr(C)]
pub struct VariationSelector {
    pub var_selector: uint24,
    pub default_uvs_offset: Offset32,
    pub non_default_uvs_offset: Offset32,
}

#[repr(C)]
#[non_exhaustive]
pub struct DefaultUvsTable {
    pub num_unicode_value_ranges: uint32,
    ranges: [UnicodeRange; 0],
}

#[repr(C)]
pub struct UnicodeRange {
    pub start_unicode_value: uint24,
    pub additional_count: u8,
}

#[repr(C)]
#[non_exhaustive]
pub struct NonDefaultUvsTable {
    pub num_uvs_mappings: uint32,
    uvs_mappings: [UvsMapping; 0],
}

#[repr(C)]
pub struct UvsMapping {
    pub unicode_value: uint24,
    pub glyph_id: uint16,
}

impl super::CmapSubtableTrait for Format14 {
    const FORMAT: u16 = 14;
}

// TODO: 'cmap' subtable format 14
