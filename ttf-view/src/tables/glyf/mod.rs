use crate::{
    tables::{Table, TableDirectory, TableError, cmap::GlyphId, loca::Loca},
    types::{Tag, int16, tags, uint16},
    util::Cursor,
};
use std::{ops::Range, ptr::NonNull};

mod composite;
mod simple;

pub use composite::*;
pub use simple::*;

impl<'a> Table<'a> for Glyf<'a> {
    const TAG: Tag = tags::glyf;
    fn new_in(dir: TableDirectory<'a>) -> Result<Self, TableError> {
        let rec = dir.table_record(Self::TAG).ok_or(TableError::NotFound)?;

        let loca = dir.loca().map_err(|_| TableError::Dependency(tags::loca))?;

        // loca is guaranteed to have at least one offset (see Loca::new_in)
        let req_len = loca.iter_raw().last().unwrap_or(0);
        if rec.length.get() < req_len {
            return Err(TableError::InvalidLen);
        }

        let ptr = NonNull::from_ref(rec.raw_as::<()>().unwrap()).cast::<Glyph>();
        let glyf = Self { ptr, loca };

        // TODO: this validation takes over 70 milliseconds on Segoe UI Emoji in debug!!!
        // Release, though, seems fine? ~5ms. I still think this needs to be optimized further.

        let num_glyphs = loca.num_glyphs();
        for glyph_id in 0..num_glyphs {
            // Glyphs can be empty, meaning no outlines
            let Some(glyph_bytes @ [_, ..]) = glyf.glyph_as_bytes(glyph_id.into()) else {
                continue;
            };

            let mut cursor = Cursor::new_in(glyph_bytes);
            // Read the base Glyph header
            let header = cursor.peek_as::<Glyph>().or(Err(TableError::InvalidLen))?;

            if header.is_simple() {
                // Read the base SimpleGlyph description
                let glyph = cursor.read_ref_as::<SimpleGlyph>().or(Err(TableError::InvalidLen))?;

                // Read the end_pts_of_contours array and move past it
                let end_pts_len = glyph.number_of_contours.get() as usize * size_of::<uint16>();
                cursor.advance_by_bytes(end_pts_len).or(Err(TableError::InvalidLen))?;

                // Ensure the end_pts_of_contours are properly sorted
                if !glyph.end_pts_of_contours().is_sorted() {
                    return Err(TableError::Malformed(&"end_pts_of_contours not sorted"));
                }

                // Read the instructions array and move past it
                let instrs = cursor.read_as::<uint16>().or(Err(TableError::InvalidLen))?;
                cursor.advance_by_bytes(instrs.get() as usize).or(Err(TableError::InvalidLen))?;

                // Attempt to process the contour points data
                let points_count = glyph.total_point_count();
                let data = process_contour_data(cursor.cast(), points_count, 0, 0)
                    .or(Err(TableError::InvalidLen))?;
                let (flags_len, xs_len, ys_len, _, extra_repeats) = data;

                // There shouldn't be any remaining repeated flags
                if extra_repeats > 0 {
                    return Err(TableError::Malformed(&"extra repeated flags"));
                }

                // Ensure that all the points data is in range of the glyph's bytes
                let points_data_len = flags_len as usize + xs_len as usize + ys_len as usize;
                cursor.advance_by_bytes(points_data_len).or(Err(TableError::InvalidLen))?;
            } else {
                // Move past the glyph header
                cursor.read_ref_as::<Glyph>().or(Err(TableError::InvalidLen))?;

                // TODO: I'm not sure if we should do cyclicity checks here?

                loop {
                    // Read the next component glyph and validate its index
                    let comp = cursor.read_ref_as::<Component>().or(Err(TableError::InvalidLen))?;
                    if comp.glyph_index.get() >= num_glyphs {
                        return Err(TableError::Malformed(&"unknown glyph id"));
                    }

                    // Move past the args and transform of the component
                    let extra_size = comp.flags().args_and_transform_size() as usize;
                    cursor.advance_by_bytes(extra_size).or(Err(TableError::InvalidLen))?;

                    // Stop reading components when there are no more left
                    if !comp.flags().intersects(ComponentFlags::MORE_COMPONENTS) {
                        break;
                    }
                }
            }
        }

        Ok(glyf)
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Glyf<'a> {
    ptr: NonNull<Glyph>,
    loca: Loca<'a>,
}

impl<'a> Glyf<'a> {
    pub fn glyph(&self, glyph_id: GlyphId) -> Option<&'a Glyph> {
        let Range { start, end } = self.loca.range(glyph_id)?;
        if start == end {
            return None;
        }
        Some(unsafe { self.ptr.byte_add(start as usize).as_ref() })
    }
    pub fn glyph_as_bytes(&self, glyph_id: GlyphId) -> Option<&'a [u8]> {
        let Range { start, end } = self.loca.range(glyph_id)?;
        if start == end {
            return Some(&[]);
        }
        let ptr = self.ptr.as_ptr().cast_const().cast::<u8>();
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

    pub const fn as_simple(&self) -> Option<&SimpleGlyph> {
        if self.is_simple() {
            Some(unsafe { std::mem::transmute::<&Self, &SimpleGlyph>(self) })
        } else {
            None
        }
    }
    pub const fn as_composite(&self) -> Option<&CompositeGlyph> {
        if self.is_composite() {
            Some(unsafe { std::mem::transmute::<&Self, &CompositeGlyph>(self) })
        } else {
            None
        }
    }
}
