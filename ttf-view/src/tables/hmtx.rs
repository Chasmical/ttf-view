use crate::{
    tables::{TableDirectoryRepr, cmap::GlyphId},
    types::{FWORD, Tag, UFWORD, tags},
};

#[repr(C)]
#[non_exhaustive]
pub struct HmtxTableRepr {
    /// Note: It's a little bit faster to work with `&[FWORD]` than with separately typed slices.
    /// See [`HmtxTableHandle::metric`] method for explanation.
    raw_words: [FWORD; 0],
    // : h_metrics: [LongHorMetricRepr; hhea().num_h_metrics],
    // : left_side_bearings: [FWORD; maxp().num_glyphs - hhea().num_h_metrics],
}
#[repr(C)]
pub struct LongHorMetricRepr {
    pub advance_width: UFWORD,
    pub lsb: FWORD,
}

impl super::Table for HmtxTableRepr {
    const TAG: Tag = tags::hmtx;
    type Handle<'a> = HmtxTableHandle<'a>;
}
impl<'a> super::TableHandle<'a> for HmtxTableHandle<'a> {
    fn in_directory(dir: &'a TableDirectoryRepr) -> Option<Self> {
        let raw_words = dir.table_raw::<HmtxTableRepr>()?.raw_words.as_ptr();
        let num_h_metrics = dir.hhea()?.number_of_h_metrics.get() as usize;
        let num_glyphs = dir.maxp()?.num_glyphs.get() as usize;

        let total_word_count = num_h_metrics + num_glyphs;
        let raw_words = unsafe { std::slice::from_raw_parts(raw_words, total_word_count) };

        Some(Self { raw_words, num_h_metrics })
    }
}

// Note: HmtxTableRepr can't provide anything on its own. We need data from two other tables:
// `number_of_h_metrics` from 'hhea' and `num_glyphs` from 'maxp' to slice the data correctly.
#[derive(Copy)]
#[derive_const(Clone)]
pub struct HmtxTableHandle<'a> {
    raw_words: &'a [FWORD],
    num_h_metrics: usize,
}

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LongHorMetric {
    pub aw: u16,
    pub lsb: i16,
}

impl LongHorMetric {
    pub const fn new(aw: u16, lsb: i16) -> Self {
        Self { aw, lsb }
    }
}
const impl From<&LongHorMetricRepr> for LongHorMetric {
    fn from(value: &LongHorMetricRepr) -> Self {
        Self { aw: value.advance_width.get(), lsb: value.lsb.get() }
    }
}

impl<'a> HmtxTableHandle<'a> {
    pub const fn num_h_metrics(&self) -> u16 {
        self.num_h_metrics as u16
    }
    pub const fn num_glyphs(&self) -> u16 {
        (self.raw_words.len() - self.num_h_metrics) as u16
    }

    const fn h_metrics(&self) -> &'a [LongHorMetricRepr] {
        unsafe { std::slice::from_raw_parts(self.raw_words.as_ptr().cast(), self.num_h_metrics) }
    }

    pub const fn last_advance_width(&self) -> Option<u16> {
        Some(self.h_metrics().last()?.advance_width.get())
    }

    pub const fn metric(&self, glyph_id: GlyphId) -> Option<LongHorMetric> {
        /// Normally, you'd check if you need to access h_metrics() or lsbs(), and then either:
        /// a) get both values from h_metrics(), or b) get lsb from lsbs(), and also maybe get
        /// the advance from h_metrics().last(), with both of these operations involving bounds
        /// checks. That's a total of 3 branches!
        ///
        /// But there's a way to combine 2 of them, leaving only 2 bounds checks:
        ///
        /// ```rs
        /// idx <= hcount-1 {
        ///     let min = idx;
        ///     // (idx*2, idx*2+1)
        ///     // (idx*2, idx+idx+1)
        ///     (min*2, min+idx+1)
        /// }
        /// idx > hcount-1 {
        ///     let min = hcount-1;
        ///     // ((hcount-1)*2, (hcount*2)+(idx-hcount))
        ///     // ((hcount-1)*2, (hcount-1)+idx+1)
        ///     (min*2, min+idx+1)
        /// }
        ///
        /// // No branching! 😎 (compiles to asm 'cmp, cmovge')
        /// let min = idx.min(hcount-1);
        /// (min*2, min+idx+1)
        /// ```
        ///
        /// Now we only have 2 bounds checks: one to ensure the glyph is in range of this cmap,
        /// and another checking if `hcount` is 0 - the only scenario in which `min` would be -1,
        /// out of range. That, of course, would mean than `min` in `min+idx+1` is `-1` too, but
        /// that's okay, - `min+1` would wrap around to 0, and all that'd remain would be `idx`.
        ///
        struct _CodeExplanation;

        let idx: usize = glyph_id.into();
        // Do the comparison as `isize`, to ensure that `-1` from `hcount-1` goes through to `min`
        let min = (idx as isize).min(self.num_h_metrics.wrapping_sub(1) as isize) as usize;

        Some(LongHorMetric {
            // Do a bounds check on min+idx+1 to check if this glyph is even represented here
            lsb: self.raw_words.get(min.wrapping_add(idx).wrapping_add(1))?.get(),

            aw: {
                if self.num_h_metrics != 0 {
                    // Unless hcount is 0, min*2 is always in valid range
                    unsafe { self.raw_words.get_unchecked(min.wrapping_mul(2)) }.get() as u16
                } else {
                    // Otherwise, return 0 as advance_width
                    0
                }
            },
        })
    }

    pub const fn iter(&self) -> Iter<'_> {
        Iter::new(*self)
    }
}

const impl<'a> IntoIterator for HmtxTableHandle<'a> {
    type Item = (GlyphId, LongHorMetric);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
const impl<'a> IntoIterator for &HmtxTableHandle<'a> {
    type Item = (GlyphId, LongHorMetric);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(*self)
    }
}

// TODO: When std::slice::Iter's Clone is constified, make the derive const
#[derive(Clone)]
pub struct Iter<'a> {
    glyph_id: u16,
    num_h_metrics: u16,
    default_aw: u16,
    raw_words: std::slice::Iter<'a, FWORD>,
}

impl<'a> Iter<'a> {
    pub const fn new(hmtx: HmtxTableHandle<'a>) -> Self {
        Self {
            glyph_id: 0,
            num_h_metrics: hmtx.num_h_metrics(),
            default_aw: hmtx.last_advance_width().unwrap_or(0),
            raw_words: hmtx.raw_words.iter(),
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (GlyphId, LongHorMetric);

    fn next(&mut self) -> Option<Self::Item> {
        let aw = if self.glyph_id < self.num_h_metrics {
            self.raw_words.next()?.get() as u16
        } else {
            self.default_aw
        };
        let lsb = self.raw_words.next()?.get();

        let id = GlyphId::new(self.glyph_id);
        self.glyph_id += 1;
        Some((id, LongHorMetric::new(aw, lsb)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        let h_metrics_left = self.num_h_metrics.saturating_sub(self.glyph_id);
        self.raw_words.len() - h_metrics_left as usize
    }
}
impl std::iter::FusedIterator for Iter<'_> {}
