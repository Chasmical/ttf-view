use crate::{
    tables::{Table, TableDirectory, TableError, cmap::GlyphId, loca::Loca},
    types::{Tag, int16, tags},
};
use std::{ops::Range, ptr::NonNull};

mod composite;
mod simple;

pub use composite::*;
pub use simple::*;

impl<'a> Table<'a> for Glyf<'a> {
    const TAG: Tag = tags::glyf;
    fn new_in(dir: &'a TableDirectory) -> Result<Self, TableError> {
        let rec = dir.table_record(Self::TAG).ok_or(TableError::NotFound)?;

        let loca = dir.loca().map_err(|_| TableError::Dependency(tags::loca))?;

        let req_len = loca.iter_raw().last().ok_or(TableError::Malformed(&"no glyphs"))?;
        if rec.length.get() < req_len {
            return Err(TableError::InvalidLen);
        }

        let ptr = NonNull::from_ref(rec.raw_as::<()>().unwrap()).cast::<u8>();
        Ok(Self { ptr, loca })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Glyf<'a> {
    ptr: NonNull<u8>,
    loca: Loca<'a>,
}

impl<'a> Glyf<'a> {
    pub fn glyph_as_bytes(&self, glyph_id: GlyphId) -> Option<&'a [u8]> {
        let Range { start, end } = self.loca.range(glyph_id)?;
        if start == end {
            return Some(&[]);
        }
        let ptr = self.ptr.as_ptr().cast_const();
        Some(unsafe { std::slice::from_raw_parts(ptr.byte_add(start as _), (end - start) as _) })
    }
}

#[repr(C)]
pub struct Glyph {
    _exhaustive_but_dont_instantiate: (),
    pub number_of_contours: int16,
    pub x_min: int16,
    pub y_min: int16,
    pub x_max: int16,
    pub y_max: int16,
    // : [more info as SimpleGlyph or CompositeGlyph],
}

impl Glyph {
    pub const fn is_simple(&self) -> bool {
        self.number_of_contours.get() >= 0
    }
    pub const fn is_composite(&self) -> bool {
        !self.is_simple()
    }
}
