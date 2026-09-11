use crate::types::{FWORD, Tag, UFWORD, int16, tags, uint16};

#[repr(C)]
pub struct HheaV0 {
    _exhaustive_but_dont_instantiate: (),
    // version any:
    pub major_version: uint16,
    pub minor_version: uint16,
}
#[repr(C)]
pub struct HheaV1 {
    v1: HheaV0,
    // version = 1.x:
    pub ascender: FWORD,
    pub descender: FWORD,
    pub line_gap: FWORD,
    pub advance_width_max: UFWORD,
    pub min_left_side_bearing: FWORD,
    pub min_right_side_bearing: FWORD,
    pub x_max_extent: FWORD,
    pub caret_slope_rise: int16,
    pub caret_slope_run: int16,
    pub caret_offset: int16,
    pub reserved0: int16,
    pub reserved1: int16,
    pub reserved2: int16,
    pub reserved3: int16,
    pub metric_data_format: int16,
    pub number_of_h_metrics: uint16,
}

const impl std::ops::Deref for HheaV1 {
    type Target = HheaV0;
    fn deref(&self) -> &Self::Target {
        &self.v1
    }
}

impl super::RawTable for HheaV0 {
    const TAG: Tag = tags::hhea;
}
impl<'a> super::Table<'a> for Hhea<'a> {
    const TAG: Tag = tags::hhea;
    fn in_directory(dir: &'a super::TableDirectory) -> Option<Self> {
        Some(Self { hhea: dir.table_raw()? })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Hhea<'a> {
    hhea: &'a HheaV0,
}

const impl<'a> std::ops::Deref for Hhea<'a> {
    type Target = &'a HheaV0;
    fn deref(&self) -> &Self::Target {
        &self.hhea
    }
}

impl<'a> Hhea<'a> {
    pub const fn v1(&self) -> Option<&'a HheaV1> {
        if self.major_version.get() == 1 {
            Some(unsafe { std::mem::transmute::<&HheaV0, &HheaV1>(self.hhea) })
        } else {
            None
        }
    }

    // version = 1.x:
    pub const fn ascender(&self) -> Option<FWORD> {
        Some(self.v1()?.ascender)
    }
    pub const fn descender(&self) -> Option<FWORD> {
        Some(self.v1()?.descender)
    }
    pub const fn line_gap(&self) -> Option<FWORD> {
        Some(self.v1()?.line_gap)
    }
    pub const fn advance_width_max(&self) -> Option<UFWORD> {
        Some(self.v1()?.advance_width_max)
    }
    pub const fn min_left_side_bearing(&self) -> Option<FWORD> {
        Some(self.v1()?.min_left_side_bearing)
    }
    pub const fn min_right_side_bearing(&self) -> Option<FWORD> {
        Some(self.v1()?.min_right_side_bearing)
    }
    pub const fn x_max_extent(&self) -> Option<FWORD> {
        Some(self.v1()?.x_max_extent)
    }
    pub const fn caret_slope_rise(&self) -> Option<int16> {
        Some(self.v1()?.caret_slope_rise)
    }
    pub const fn caret_slope_run(&self) -> Option<int16> {
        Some(self.v1()?.caret_slope_run)
    }
    pub const fn caret_offset(&self) -> Option<int16> {
        Some(self.v1()?.caret_offset)
    }
    pub const fn metric_data_format(&self) -> Option<int16> {
        Some(self.v1()?.metric_data_format)
    }
    pub const fn number_of_h_metrics(&self) -> Option<uint16> {
        Some(self.v1()?.number_of_h_metrics)
    }
}

impl std::fmt::Debug for Hhea<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("HheaTable");
        f.field("major_version", &self.major_version.get())
            .field("minor_version", &self.minor_version.get());

        if let Some(v1) = self.v1() {
            f.field("ascender", &v1.ascender.get());
            f.field("descender", &v1.descender.get());
            f.field("line_gap", &v1.line_gap.get());
            f.field("advance_width_max", &v1.advance_width_max.get());
            f.field("min_left_side_bearing", &v1.min_left_side_bearing.get());
            f.field("min_right_side_bearing", &v1.min_right_side_bearing.get());
            f.field("x_max_extent", &v1.x_max_extent.get());
            f.field("caret_slope_rise", &v1.caret_slope_rise.get());
            f.field("caret_slope_run", &v1.caret_slope_run.get());
            f.field("caret_offset", &v1.caret_offset.get());
            f.field("reserved0", &v1.reserved0.get());
            f.field("reserved1", &v1.reserved1.get());
            f.field("reserved2", &v1.reserved2.get());
            f.field("reserved3", &v1.reserved3.get());
            f.field("metric_data_format", &v1.metric_data_format.get());
            f.field("number_of_h_metrics", &v1.number_of_h_metrics.get());
        }
        f.finish()
    }
}

// Monomorphize all fmts to the above fmt for Hhea<'a>
impl std::fmt::Debug for HheaV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Hhea { hhea: self }.fmt(f)
    }
}
impl std::fmt::Debug for HheaV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Hhea { hhea: self }.fmt(f)
    }
}
