use crate::tables::cmap::GlyphId;

#[repr(C)]
pub struct Format0 {
    pub glyph_id_array: [u8; 256],
}

impl super::CmapSubtableTrait for Format0 {
    const FORMAT: u16 = 0;
}

impl Format0 {
    pub const fn map(&self, codepoint: u8) -> GlyphId {
        self.glyph_id_array[codepoint as usize].into()
    }
}
