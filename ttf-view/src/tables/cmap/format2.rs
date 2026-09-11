use crate::{
    tables::cmap::GlyphId,
    types::{int16, uint16},
};

#[repr(C)]
#[non_exhaustive]
pub struct Format2 {
    pub sub_header_keys: [uint16; 256],
    sub_headers: [SubHeader; 0],
    glyph_id_array: [uint16; 0],
}

#[repr(C)]
#[non_exhaustive]
pub struct SubHeader {
    pub first_code: uint16,
    pub entry_count: uint16,
    pub id_delta: int16,
    pub id_range_offset: uint16,
}

impl super::CmapSubtableTrait for Format2 {
    const FORMAT: u16 = 2;
}

impl Format2 {
    pub fn sub_headers(&self) -> &[SubHeader] {
        let max_offset = self.sub_header_keys.iter().max().unwrap().get();
        let count = (max_offset / 8) + 1;
        unsafe { std::slice::from_raw_parts(self.sub_headers.as_ptr(), count as _) }
    }
    pub const fn sub_header(&self, high_byte: u8) -> &SubHeader {
        let offset = self.sub_header_keys[high_byte as usize].get();
        unsafe { &*self.sub_headers.as_ptr().byte_add(offset as usize) }
    }
    pub const fn sub_header_zero(&self) -> &SubHeader {
        unsafe { &*self.sub_headers.as_ptr() }
    }

    pub const fn map_one(&self, single_byte: u8) -> GlyphId {
        self.sub_header_zero().map(single_byte)
    }
    pub const fn map_two(&self, high_byte: u8, low_byte: u8) -> GlyphId {
        self.sub_header(high_byte).map(low_byte)
    }
}

impl SubHeader {
    pub const fn map(&self, low_byte: u8) -> GlyphId {
        let offset = (low_byte as u16).checked_sub(self.first_code.get())?;
        if offset > self.entry_count.get() {
            return GlyphId::NOTDEF;
        }

        let mut result = unsafe {
            (&*std::ptr::from_ref(&self.id_range_offset)
                .byte_add(self.id_range_offset.get() as _)
                .add(offset as _))
                .get()
        };
        if result != 0 {
            result = result.wrapping_add_signed(self.id_delta.get());
        }

        GlyphId::new(result)
    }
}
