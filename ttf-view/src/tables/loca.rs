use crate::{
    tables::{Table, TableDirectory, TableError, cmap::GlyphId},
    types::{Offset16, Offset32, Tag, tags},
    util::{PackedDualIter, custom_iterator, fmt_with},
};
use std::{num::NonZero, ops::Range, ptr::NonNull};

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i16)]
#[non_exhaustive]
pub enum LocaFormat {
    Short = 0,
    Long = 1,
}

impl LocaFormat {
    // Note: this fn is not public because LocaFormat is not exhaustive.
    const fn stride(&self) -> u8 {
        match self {
            Self::Short => 2,
            Self::Long => 4,
        }
    }
}
const impl TryFrom<i16> for LocaFormat {
    type Error = ();
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Short,
            1 => Self::Long,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocaOffsets<'a> {
    Short(&'a [Offset16]),
    Long(&'a [Offset32]),
}

impl<'a> Table<'a> for Loca<'a> {
    const TAG: Tag = tags::loca;
    fn new_in(dir: TableDirectory<'a>) -> Result<Self, TableError> {
        let rec = dir.table_record(Self::TAG).ok_or(TableError::NotFound)?;

        let format = dir
            .head()
            .map_err(|_| TableError::Dependency(tags::head))?
            .index_to_loc_format()
            .ok_or(TableError::DependencyError(&"index_to_loc_format not found"))
            .and_then(|format| {
                LocaFormat::try_from(format).map_err(|_| TableError::UnknownFormat)
            })?;

        let num_glyphs = dir
            .maxp()
            .map_err(|_| TableError::Dependency(tags::maxp))?
            .num_glyphs()
            .ok_or(TableError::DependencyError(&"num_glyphs not found"))?;

        let ptr = NonNull::from_ref(rec.raw_as::<()>().unwrap());

        let req_len = (num_glyphs as usize + 1) * format.stride() as usize;
        if (rec.length.get() as usize) < req_len {
            return Err(TableError::InvalidLen);
        }

        let loca = Self { ptr, num_glyphs, format, _phantom: Default::default() };

        if !loca.iter_raw().is_sorted() {
            return Err(TableError::Malformed(&"offsets not sorted"));
        }

        Ok(loca)
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Loca<'a> {
    ptr: NonNull<()>,
    num_glyphs: u16,
    format: LocaFormat,
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
        let len = self.num_glyphs as usize + 1;
        match self.format {
            LocaFormat::Short => LocaOffsets::Short(unsafe {
                std::slice::from_raw_parts(self.ptr.as_ptr().cast(), len)
            }),
            LocaFormat::Long => LocaOffsets::Long(unsafe {
                std::slice::from_raw_parts(self.ptr.as_ptr().cast(), len)
            }),
        }
    }

    pub fn range(&self, glyph_id: GlyphId) -> Option<Range<u32>> {
        Some(self.iter().nth(glyph_id.get() as usize)?.1)
    }

    pub fn iter_raw(&self) -> RawIter<'a> {
        RawIter::new(*self)
    }
    pub fn iter(&self) -> Iter<'a> {
        Iter::new(*self)
    }
}

#[derive(Clone)]
pub struct RawIter<'a> {
    inner: PackedDualIter<'a, u32, Offset32, Offset16>,
}
impl<'a> RawIter<'a> {
    pub fn new(loca: Loca<'a>) -> Self {
        let inner = match loca.offsets() {
            LocaOffsets::Long(offsets) => PackedDualIter::new_a(offsets),
            LocaOffsets::Short(offsets) => PackedDualIter::new_b(offsets),
        };
        Self { inner }
    }
}
custom_iterator!(RawIter<'a> as this {
    type Item = u32;
    map: |x| x;
});

#[derive(Clone)]
pub struct Iter<'a> {
    inner: RawIter<'a>,
    glyph_id: u16,
    prev: u32,
}
impl<'a> Iter<'a> {
    pub fn new(loca: Loca<'a>) -> Self {
        let mut inner = RawIter::new(loca);
        // loca is guaranteed to have at least one offset (see Loca::new_in)
        let first = inner.next().unwrap_or(0);
        Self { inner, glyph_id: 0, prev: first }
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = (GlyphId, Range<u32>);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let ret = (self.glyph_id.into(), self.prev..next);
        self.prev = next;
        // GlyphId(65535) is the last glyph, so it's okay to wrap here
        self.glyph_id = self.glyph_id.wrapping_add(1);
        Some(ret)
    }
    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        let advance = self.len().min(n);
        if advance > 0 {
            // Read the offset before the one we're advancing to
            self.prev = self.inner.nth(advance - 1).unwrap();
        }
        NonZero::new(n - advance).map_or(Ok(()), Err)
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.advance_by(n).ok()?;
        self.next()
    }
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: std::ops::Try<Output = B>,
    {
        self.inner.try_fold(init, |init, next| {
            let ret = (self.glyph_id.into(), self.prev..next);
            self.prev = next;
            self.glyph_id = self.glyph_id.wrapping_add(1);
            f(init, ret)
        })
    }
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where F: FnMut(B, Self::Item) -> B {
        self.try_fold(init, |init, x| Ok::<_, !>(f(init, x))).unwrap()
    }
    fn last(mut self) -> Option<Self::Item> {
        self.nth(self.len() - 1)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<'a> std::iter::FusedIterator for Iter<'a> {}

impl std::fmt::Debug for Loca<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("loca");
        f.field("index_to_loc_format", fmt_with!("{} ({:?})", self.format as i16, self.format));
        f.field("num_glyphs", &self.num_glyphs());

        f.field("offsets", fmt_with! { |f| {
            write!(f, "{{")?;

            let id_width = self.num_glyphs.checked_ilog10().unwrap_or(0) as usize + 1;
            let last_offset = self.iter_raw().last();
            let offset_width =
                last_offset.and_then(|x| x.checked_ilog(16)).unwrap_or(0) as usize + 3;

            for (glyph_id, range) in self.iter() {
                if glyph_id.get() % 5 == 0 {
                    write!(f, "\n   ")?;
                }
                write!(f, " {:id_width$}: {:#0offset_width$X},", glyph_id, range.start)?;
            }
            if let Some(last_offset) = last_offset {
                write!(f, " {:>id_width$}: {:#0offset_width$X},", "_", last_offset)?;
            }

            write!(f, "\n}}")
        } });

        f.finish()
    }
}
