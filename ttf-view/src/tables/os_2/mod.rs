#![allow(non_camel_case_types)]
use crate::{
    tables::TableDirectory,
    types::{FWORD, Tag, UFWORD, int16, tags, uint16, uint32},
};

mod code_pages;
mod panose;
mod unicode_ranges;

pub use code_pages::*;
pub use panose::*;
pub use unicode_ranges::*;

#[repr(C)]
pub struct Os_2Base {
    _exhaustive_but_dont_instantiate: (),
    // version any:
    pub version: uint16,
    pub x_avg_char_width: FWORD,
    pub us_weight_class: uint16,
    pub us_width_class: uint16,
    pub fs_type: uint16,
    pub y_subscript_x_size: FWORD,
    pub y_subscript_y_size: FWORD,
    pub y_subscript_x_offset: FWORD,
    pub y_subscript_y_offset: FWORD,
    pub y_superscript_x_size: FWORD,
    pub y_superscript_y_size: FWORD,
    pub y_superscript_x_offset: FWORD,
    pub y_superscript_y_offset: FWORD,
    pub y_strikeout_size: FWORD,
    pub y_strikeout_position: FWORD,
    pub s_family_class: int16,
    pub panose: Panose,
    pub ul_unicode_range_1: uint32,
    pub ul_unicode_range_2: uint32,
    pub ul_unicode_range_3: uint32,
    pub ul_unicode_range_4: uint32,
    pub ach_vend_id: Tag,
    pub fs_selection: uint16,
    pub us_first_char_index: uint16,
    pub us_last_char_index: uint16,
}
#[repr(C)]
pub struct Os_2V0 {
    base: Os_2Base,
    // version ≥ 0:
    // Apple's docs attribute these 5 fields to v1 instead of v0, so they could be missing in
    // some malformed fonts. For the purposes of versioning such a table is considered Os_2Base,
    // and you need to explicitly check that a table is a properly defined v0 table.
    pub s_typo_ascender: FWORD,
    pub s_typo_descender: FWORD,
    pub s_typo_line_gap: FWORD,
    pub us_win_ascent: UFWORD,
    pub us_win_descent: UFWORD,
}
#[repr(C)]
pub struct Os_2V1 {
    v0: Os_2V0,
    // version ≥ 1:
    pub ul_code_page_range_1: uint32,
    pub ul_code_page_range_2: uint32,
}
#[repr(C)]
pub struct Os_2V4 {
    v1: Os_2V1,
    // version ≥ 4:
    pub sx_height: FWORD,
    pub s_cap_height: FWORD,
    pub us_default_char: uint16,
    pub us_break_char: uint16,
    pub us_max_content: uint16,
}
#[repr(C)]
pub struct Os_2V5 {
    v4: Os_2V4,
    // version ≥ 5:
    pub us_lower_optical_point_size: uint16,
    pub us_upper_optical_point_size: uint16,
}

const impl std::ops::Deref for Os_2V0 {
    type Target = Os_2Base;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
const impl std::ops::Deref for Os_2V1 {
    type Target = Os_2V0;
    fn deref(&self) -> &Self::Target {
        &self.v0
    }
}
const impl std::ops::Deref for Os_2V4 {
    type Target = Os_2V1;
    fn deref(&self) -> &Self::Target {
        &self.v1
    }
}
const impl std::ops::Deref for Os_2V5 {
    type Target = Os_2V4;
    fn deref(&self) -> &Self::Target {
        &self.v4
    }
}

impl super::RawTable for Os_2Base {
    const TAG: Tag = tags::OS_2;
}
impl<'a> super::Table<'a> for Os_2<'a> {
    const TAG: Tag = tags::OS_2;
    fn in_directory(dir: &'a TableDirectory) -> Option<Self> {
        let record = dir.table_record(tags::OS_2)?;
        Some(Self { base: record.table_as()?, len: record.length.get() })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Os_2<'a> {
    base: &'a Os_2Base,
    len: u32,
}

const impl<'a> std::ops::Deref for Os_2<'a> {
    type Target = &'a Os_2Base;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> Os_2<'a> {
    pub const fn v0(&self) -> Option<&'a Os_2V0> {
        const V0_SIZE: u32 = size_of::<Os_2V0>() as u32;
        if self.len >= V0_SIZE {
            Some(unsafe { std::mem::transmute::<&Os_2Base, &Os_2V0>(self.base) })
        } else {
            None
        }
    }
    pub const fn v1(&self) -> Option<&'a Os_2V1> {
        if self.version.get() >= 1 {
            Some(unsafe { std::mem::transmute::<&Os_2Base, &Os_2V1>(self.base) })
        } else {
            None
        }
    }
    pub const fn v4(&self) -> Option<&'a Os_2V4> {
        if self.version.get() >= 4 {
            Some(unsafe { std::mem::transmute::<&Os_2Base, &Os_2V4>(self.base) })
        } else {
            None
        }
    }
    pub const fn v5(&self) -> Option<&'a Os_2V5> {
        if self.version.get() >= 5 {
            Some(unsafe { std::mem::transmute::<&Os_2Base, &Os_2V5>(self.base) })
        } else {
            None
        }
    }

    // version any:
    pub const fn unicode_range(&self) -> UnicodeRanges {
        UnicodeRanges::from_parts(
            self.ul_unicode_range_1.get(),
            self.ul_unicode_range_2.get(),
            self.ul_unicode_range_3.get(),
            self.ul_unicode_range_4.get(),
        )
    }

    // version ≥ 0:
    pub const fn s_typo_ascender(&self) -> Option<FWORD> {
        Some(self.v0()?.s_typo_ascender)
    }
    pub const fn s_typo_descender(&self) -> Option<FWORD> {
        Some(self.v0()?.s_typo_descender)
    }
    pub const fn s_typo_line_gap(&self) -> Option<FWORD> {
        Some(self.v0()?.s_typo_line_gap)
    }
    pub const fn us_win_ascent(&self) -> Option<UFWORD> {
        Some(self.v0()?.us_win_ascent)
    }
    pub const fn us_win_descent(&self) -> Option<UFWORD> {
        Some(self.v0()?.us_win_descent)
    }

    // version ≥ 1:
    pub const fn ul_code_page_range_1(&self) -> Option<uint32> {
        Some(self.v1()?.ul_code_page_range_1)
    }
    pub const fn ul_code_page_range_2(&self) -> Option<uint32> {
        Some(self.v1()?.ul_code_page_range_2)
    }
    pub const fn code_page_range(&self) -> Option<CodePages> {
        let v1 = self.v1()?;
        Some(CodePages::from_parts(v1.ul_code_page_range_1.get(), v1.ul_code_page_range_2.get()))
    }

    // version ≥ 4:
    pub const fn sx_height(&self) -> Option<FWORD> {
        Some(self.v4()?.sx_height)
    }
    pub const fn s_cap_height(&self) -> Option<FWORD> {
        Some(self.v4()?.s_cap_height)
    }
    pub const fn us_default_char(&self) -> Option<uint16> {
        Some(self.v4()?.us_default_char)
    }
    pub const fn us_break_char(&self) -> Option<uint16> {
        Some(self.v4()?.us_break_char)
    }
    pub const fn us_max_content(&self) -> Option<uint16> {
        Some(self.v4()?.us_max_content)
    }

    // version ≥ 5:
    pub const fn us_lower_optical_point_size(&self) -> Option<uint16> {
        Some(self.v5()?.us_lower_optical_point_size)
    }
    pub const fn us_upper_optical_point_size(&self) -> Option<uint16> {
        Some(self.v5()?.us_upper_optical_point_size)
    }
}

impl std::fmt::Debug for Os_2<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("OS/2");

        f.field("version", &self.version.get())
            .field("x_avg_char_width", &self.x_avg_char_width.get())
            .field("us_weight_class", &self.us_weight_class.get())
            .field("us_width_class", &self.us_width_class.get())
            .field_with("fs_type", |f| write!(f, "{:#010b}", self.fs_type))
            // TODO: combine into ScriptMetrics struct (and add getter)
            .field("y_subscript_x_size", &self.y_subscript_x_size.get())
            .field("y_subscript_y_size", &self.y_subscript_y_size.get())
            .field("y_subscript_x_offset", &self.y_subscript_x_offset.get())
            .field("y_subscript_y_offset", &self.y_subscript_y_offset.get())
            .field("y_superscript_x_size", &self.y_superscript_x_size.get())
            .field("y_superscript_y_size", &self.y_superscript_y_size.get())
            .field("y_superscript_x_offset", &self.y_superscript_x_offset.get())
            .field("y_superscript_y_offset", &self.y_superscript_y_offset.get())
            // TODO: combine into StrikeoutMetrics (and add getter)
            .field("y_strikeout_size", &self.y_strikeout_size.get())
            .field("y_strikeout_position", &self.y_strikeout_position.get())
            .field("s_family_class", &self.s_family_class.get())
            .field("panose", &self.panose)
            // TODO: impl Debug for UnicodeRanges
            .field("ul_unicode_range_1", &self.ul_unicode_range_1.get())
            .field("ul_unicode_range_2", &self.ul_unicode_range_2.get())
            .field("ul_unicode_range_3", &self.ul_unicode_range_3.get())
            .field("ul_unicode_range_4", &self.ul_unicode_range_4.get())
            .field("ach_vend_id", &self.ach_vend_id)
            .field("fs_selection", &self.fs_selection.get())
            .field("us_first_char_index", &self.us_first_char_index.get())
            .field("us_last_char_index", &self.us_last_char_index.get());

        if let Some(v0) = self.v0() {
            // TODO: combine into TypoMetrics (and add getter) (maybe?)
            f.field("s_typo_ascender", &v0.s_typo_ascender.get())
                .field("s_typo_descender", &v0.s_typo_descender.get())
                .field("s_typo_line_gap", &v0.s_typo_line_gap.get())
                .field("us_win_ascent", &v0.us_win_ascent.get())
                .field("us_win_descent", &v0.us_win_descent.get());
        }
        if let Some(v1) = self.v1() {
            // TODO: impl Debug for CodePages
            f.field("ul_code_page_range_1", &v1.ul_code_page_range_1.get())
                .field("ul_code_page_range_2", &v1.ul_code_page_range_2.get());
        }
        if let Some(v4) = self.v4() {
            f.field("sx_height", &v4.sx_height.get())
                .field("s_cap_height", &v4.s_cap_height.get())
                // TODO: display something like 'X' (U+CODE) (and add getter () -> char)
                .field("us_default_char", &v4.us_default_char.get())
                .field("us_break_char", &v4.us_break_char.get())
                .field("us_max_content", &v4.us_max_content.get());
        }
        if let Some(v5) = self.v5() {
            f.field("us_lower_optical_point_size", &v5.us_lower_optical_point_size.get())
                .field("us_upper_optical_point_size", &v5.us_upper_optical_point_size.get());
        }

        f.finish()
    }
}

// Monomorphize all fmts to the above fmt for Os_2<'a>
impl std::fmt::Debug for Os_2Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Os_2 { base: self, len: size_of_val(self) as u32 }.fmt(f)
    }
}
impl std::fmt::Debug for Os_2V0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Os_2 { base: self, len: size_of_val(self) as u32 }.fmt(f)
    }
}
impl std::fmt::Debug for Os_2V1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Os_2 { base: self, len: size_of_val(self) as u32 }.fmt(f)
    }
}
impl std::fmt::Debug for Os_2V4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Os_2 { base: self, len: size_of_val(self) as u32 }.fmt(f)
    }
}
impl std::fmt::Debug for Os_2V5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Os_2 { base: self, len: size_of_val(self) as u32 }.fmt(f)
    }
}
