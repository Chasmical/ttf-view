use crate::tables::cmap::GlyphId;

#[repr(C)]
pub struct CmapSubtable0 {
    pub glyph_id_array: [u8; 256],
}

impl CmapSubtable0 {
    pub const fn map(&self, codepoint: u8) -> GlyphId {
        self.glyph_id_array[codepoint as usize].into()
    }
}
