use crate::{
    tables::{Table, TableDirectory, TableError, cmap::GlyphId},
    types::{Offset16, Offset32, Tag, tags},
};
use std::{fmt::Debug, ops::Range};

#[repr(C)]
struct LocaRaw {
    short_offsets: [Offset16; 0],
    long_offsets: [Offset32; 0],
}

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i16)]
#[non_exhaustive]
pub enum LocaFormat {
    Short = 0,
    Long = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocaOffsets<'a> {
    Short(&'a [Offset16]),
    Long(&'a [Offset32]),
}

impl<'a> Table<'a> for Loca<'a> {
    const TAG: Tag = tags::loca;
    fn new_in(dir: &'a TableDirectory) -> Result<Self, TableError> {
        let rec = dir.table_record(Self::TAG).ok_or(TableError::NotFound)?;

        let format = dir
            .head()
            .map_err(|_| TableError::Dependency(tags::head))?
            .index_to_loc_format()
            .ok_or(TableError::DependencyError(&"index_to_loc_format not found"))?
            .get();

        let num_glyphs = dir
            .maxp()
            .map_err(|_| TableError::Dependency(tags::maxp))?
            .num_glyphs()
            .ok_or(TableError::DependencyError(&"num_glyphs not found"))?
            .get();

        let loca = rec.raw_as::<LocaRaw>().unwrap();

        let (format, ptr) = match format {
            0 => {
                let req_len = (num_glyphs as usize + 1) * size_of::<Offset16>();
                if (rec.length.get() as usize) < req_len {
                    return Err(TableError::InvalidLen);
                }
                (LocaFormat::Short, loca.short_offsets.as_ptr().cast())
            },
            1 => {
                let req_len = (num_glyphs as usize + 1) * size_of::<Offset32>();
                if (rec.length.get() as usize) < req_len {
                    return Err(TableError::InvalidLen);
                }
                (LocaFormat::Long, loca.long_offsets.as_ptr().cast())
            },
            _ => return Err(TableError::UnknownFormat),
        };

        Ok(Self { num_glyphs, format, ptr, _phantom: Default::default() })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Loca<'a> {
    num_glyphs: u16,
    format: LocaFormat,
    ptr: *const (),
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Loca<'a> {
    pub const fn num_glyphs(&self) -> u16 {
        self.num_glyphs
    }
    pub const fn format(&self) -> LocaFormat {
        self.format
    }
    pub const fn offsets(&self) -> LocaOffsets<'a> {
        match self.format {
            LocaFormat::Short => LocaOffsets::Short(unsafe {
                std::slice::from_raw_parts(self.ptr.cast(), self.num_glyphs as usize)
            }),
            LocaFormat::Long => LocaOffsets::Long(unsafe {
                std::slice::from_raw_parts(self.ptr.cast(), self.num_glyphs as usize)
            }),
        }
    }

    pub const fn as_short(&self) -> Option<&[Offset16]> {
        (self.format == LocaFormat::Short).then(const || unsafe {
            std::slice::from_raw_parts(self.ptr.cast(), self.num_glyphs as usize + 1)
        })
    }
    pub const fn as_long(&self) -> Option<&[Offset32]> {
        (self.format == LocaFormat::Long).then(const || unsafe {
            std::slice::from_raw_parts(self.ptr.cast(), self.num_glyphs as usize + 1)
        })
    }

    pub const fn range(&self, glyph_id: GlyphId) -> Option<Range<u32>> {
        if glyph_id.get() >= self.num_glyphs {
            return None;
        }

        Some(match self.format {
            LocaFormat::Short => unsafe { loca_range_short(self.ptr, glyph_id) },
            LocaFormat::Long => unsafe { loca_range_long(self.ptr, glyph_id) },
        })
    }
    pub const fn offset(&self, glyph_id: GlyphId) -> Option<u32> {
        Some(self.range(glyph_id)?.start)
    }

    pub const fn iter(&self) -> Iter<'_> {
        Iter::new(*self)
    }
}

const unsafe fn loca_range_short(ptr: *const (), glyph_id: GlyphId) -> Range<u32> {
    let idx = usize::from(glyph_id);
    unsafe {
        let this = (&*ptr.cast::<Offset16>().add(idx)).get() as u32;
        let next = (&*ptr.cast::<Offset16>().add(idx + 1)).get() as u32;
        this..next
    }
}
const unsafe fn loca_range_long(ptr: *const (), glyph_id: GlyphId) -> Range<u32> {
    let idx = usize::from(glyph_id);
    unsafe {
        let this = (&*ptr.cast::<Offset32>().add(idx)).get();
        let next = (&*ptr.cast::<Offset32>().add(idx + 1)).get();
        this..next
    }
}

const impl<'a> IntoIterator for Loca<'a> {
    type Item = (GlyphId, Range<u32>);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
const impl<'a> IntoIterator for &Loca<'a> {
    type Item = (GlyphId, Range<u32>);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(*self)
    }
}

// TODO: When std::slice::Iter's Clone is constified, make the derive const
#[derive(Clone)]
pub struct Iter<'a> {
    glyph_id: u16,
    num_glyphs: u16,
    format: LocaFormat,
    ptr: *const (),
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Iter<'a> {
    pub const fn new(loca: Loca<'a>) -> Self {
        Self {
            glyph_id: 0,
            num_glyphs: loca.num_glyphs,
            format: loca.format,
            ptr: loca.ptr,
            _phantom: Default::default(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (GlyphId, Range<u32>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.glyph_id >= self.num_glyphs {
            return None;
        }

        let idx = GlyphId::new(self.glyph_id);
        self.glyph_id += 1;

        let range = match self.format {
            LocaFormat::Short => unsafe { loca_range_short(self.ptr, idx) },
            LocaFormat::Long => unsafe { loca_range_long(self.ptr, idx) },
        };

        Some((idx, range))
    }
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.num_glyphs <= self.glyph_id {
            return None;
        }

        self.num_glyphs -= 1;
        let idx = GlyphId::new(self.num_glyphs);

        let range = match self.format {
            LocaFormat::Short => unsafe { loca_range_short(self.ptr, idx) },
            LocaFormat::Long => unsafe { loca_range_long(self.ptr, idx) },
        };

        Some((idx, range))
    }
}
impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        (self.num_glyphs - self.glyph_id) as _
    }
}

impl std::fmt::Debug for Loca<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("loca");

        f.field_with("index_to_loc_format", |f| {
            write!(f, "{} ({:?})", self.format as i16, self.format)
        });
        f.field("num_glyphs", &self.num_glyphs());

        f.field_with("offsets", |f| {
            write!(f, "[")?;

            let id_width = self.num_glyphs.ilog10() as usize + 1;
            let last_range = self.iter().last();
            let offset_width = last_range.clone().map_or(0, |x| x.1.start.ilog(16) as usize + 3);

            for (glyph_id, offset) in self.iter() {
                if glyph_id.get() % 5 == 0 {
                    write!(f, "\n   ")?;
                }
                write!(f, " {:id_width$}: {:#0offset_width$X},", glyph_id, offset.start)?;
            }
            if let Some(last_range) = last_range {
                write!(f, " {:>id_width$}: {:#0offset_width$X},", "_", last_range.1.end)?;
            }

            write!(f, "\n]")
        });

        f.finish()
    }
}
