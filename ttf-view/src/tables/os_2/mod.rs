#![allow(non_camel_case_types)]
use crate::{
    tables::{Table, TableDirectory, TableError},
    types::{FWORD, Tag, UFWORD, int16, tags, uint16, uint32},
    util::fmt_with,
};
use std::ops::Range;

mod code_pages;
mod metrics_and_flags;
mod unicode_ranges;

pub use code_pages::*;
pub use metrics_and_flags::*;
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
pub struct Os_2V2 {
    v1: Os_2V1,
    // version ≥ 2:
    pub sx_height: FWORD,
    pub s_cap_height: FWORD,
    pub us_default_char: uint16,
    pub us_break_char: uint16,
    pub us_max_context: uint16,
}
#[repr(C)]
pub struct Os_2V5 {
    v2: Os_2V2,
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
const impl std::ops::Deref for Os_2V2 {
    type Target = Os_2V1;
    fn deref(&self) -> &Self::Target {
        &self.v1
    }
}
const impl std::ops::Deref for Os_2V5 {
    type Target = Os_2V2;
    fn deref(&self) -> &Self::Target {
        &self.v2
    }
}

impl<'a> Table<'a> for Os_2<'a> {
    const TAG: Tag = tags::OS_2;
    fn new_in(dir: &'a TableDirectory) -> Result<Self, TableError> {
        let rec = dir.table_record(tags::OS_2).ok_or(TableError::NotFound)?;
        let base = rec.raw_as::<Os_2Base>().ok_or(TableError::InvalidLen)?;

        let required_len = match base.version.get() {
            0 => size_of::<Os_2Base>() as u32,
            1 => size_of::<Os_2V1>() as u32,
            2..=4 => size_of::<Os_2V2>() as u32,
            5.. => size_of::<Os_2V5>() as u32,
        };
        if rec.length.get() < required_len {
            return Err(TableError::InvalidLen);
        }

        let os_2 = Self { base, len: rec.length.get() };

        // Validate us_{first,last}_char_index so we can return them as chars safely
        if char::from_u32(os_2.us_first_char_index.get() as u32).is_none() {
            return Err(TableError::Malformed(&"us_default_char not unicode"));
        }
        if char::from_u32(os_2.us_last_char_index.get() as u32).is_none() {
            return Err(TableError::Malformed(&"us_break_char not unicode"));
        }

        // Validate us_{default,break}_char so we can return them as chars safely
        if let Some(v2) = os_2.v2() {
            if char::from_u32(v2.us_default_char.get() as u32).is_none() {
                return Err(TableError::Malformed(&"us_default_char not unicode"));
            }
            if char::from_u32(v2.us_break_char.get() as u32).is_none() {
                return Err(TableError::Malformed(&"us_break_char not unicode"));
            }
        }

        Ok(os_2)
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

impl Os_2Base {
    pub const fn embedding_rights(&self) -> EmbeddingRights {
        EmbeddingRights::new_truncating(self.fs_type.get(), self.version.get())
    }
    pub const fn subscript_metrics(&self) -> ScriptMetrics {
        ScriptMetrics {
            x_size: self.y_subscript_x_size.get(),
            y_size: self.y_subscript_y_size.get(),
            x_offset: self.y_subscript_x_offset.get(),
            y_offset: self.y_subscript_y_offset.get(),
        }
    }
    pub const fn superscript_metrics(&self) -> ScriptMetrics {
        ScriptMetrics {
            x_size: self.y_superscript_x_size.get(),
            y_size: self.y_superscript_y_size.get(),
            x_offset: self.y_superscript_x_offset.get(),
            y_offset: self.y_superscript_y_offset.get(),
        }
    }
    pub const fn strikeout_metrics(&self) -> LineMetrics {
        LineMetrics {
            thickness: self.y_strikeout_size.get(),
            position: self.y_strikeout_position.get(),
        }
    }
    pub const fn unicode_range(&self) -> UnicodeRanges {
        UnicodeRanges::from_parts(
            self.ul_unicode_range_1.get(),
            self.ul_unicode_range_2.get(),
            self.ul_unicode_range_3.get(),
            self.ul_unicode_range_4.get(),
        )
    }
    pub const fn selection(&self) -> SelectionFlags {
        SelectionFlags::new_truncating(self.fs_selection.get(), self.version.get())
    }
    pub const fn first_char(&self) -> Option<char> {
        let ch = unsafe { char::from_u32_unchecked(self.us_first_char_index.get() as u32) };
        if ch != '\u{ffff}' { Some(ch) } else { None }
    }
    pub const fn last_char(&self) -> Option<char> {
        let ch = unsafe { char::from_u32_unchecked(self.us_last_char_index.get() as u32) };
        if ch != '\u{ffff}' { Some(ch) } else { None }
    }
    pub const fn char_range(&self) -> (Option<char>, Option<char>) {
        (self.first_char(), self.last_char())
    }
}
impl Os_2V0 {
    pub const fn typographic_metrics(&self) -> TypographicMetrics {
        TypographicMetrics {
            ascender: self.s_typo_ascender.get(),
            descender: self.s_typo_descender.get(),
            line_gap: self.s_typo_line_gap.get(),
        }
    }
    pub const fn windows_metrics(&self) -> WindowsMetrics {
        WindowsMetrics { ascent: self.us_win_ascent.get(), descent: self.us_win_descent.get() }
    }
}
impl Os_2V1 {
    pub const fn code_page_range(&self) -> CodePages {
        CodePages::from_parts(self.ul_code_page_range_1.get(), self.ul_code_page_range_2.get())
    }
}
impl Os_2V2 {
    pub const fn default_char(&self) -> char {
        unsafe { char::from_u32_unchecked(self.us_default_char.get() as u32) }
    }
    pub const fn break_char(&self) -> char {
        unsafe { char::from_u32_unchecked(self.us_break_char.get() as u32) }
    }
}
impl Os_2V5 {
    pub const fn optical_point_size(&self) -> Range<u16> {
        self.us_lower_optical_point_size.get()..self.us_upper_optical_point_size.get()
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
    pub const fn v2(&self) -> Option<&'a Os_2V2> {
        if self.version.get() >= 2 {
            Some(unsafe { std::mem::transmute::<&Os_2Base, &Os_2V2>(self.base) })
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
    pub const fn typo_metrics(&self) -> Option<TypographicMetrics> {
        Some(self.v0()?.typographic_metrics())
    }
    pub const fn us_win_ascent(&self) -> Option<UFWORD> {
        Some(self.v0()?.us_win_ascent)
    }
    pub const fn us_win_descent(&self) -> Option<UFWORD> {
        Some(self.v0()?.us_win_descent)
    }
    pub const fn win_metrics(&self) -> Option<WindowsMetrics> {
        Some(self.v0()?.windows_metrics())
    }

    // version ≥ 1:
    pub const fn ul_code_page_range_1(&self) -> Option<u32> {
        Some(self.v1()?.ul_code_page_range_1.get())
    }
    pub const fn ul_code_page_range_2(&self) -> Option<u32> {
        Some(self.v1()?.ul_code_page_range_2.get())
    }
    pub const fn code_page_range(&self) -> Option<CodePages> {
        Some(self.v1()?.code_page_range())
    }

    // version ≥ 2:
    pub const fn sx_height(&self) -> Option<FWORD> {
        Some(self.v2()?.sx_height)
    }
    pub const fn s_cap_height(&self) -> Option<FWORD> {
        Some(self.v2()?.s_cap_height)
    }
    pub const fn us_default_char(&self) -> Option<u16> {
        Some(self.v2()?.us_default_char.get())
    }
    pub const fn default_char(&self) -> Option<char> {
        Some(self.v2()?.default_char())
    }
    pub const fn us_break_char(&self) -> Option<u16> {
        Some(self.v2()?.us_break_char.get())
    }
    pub const fn break_char(&self) -> Option<char> {
        Some(self.v2()?.break_char())
    }
    pub const fn us_max_context(&self) -> Option<u16> {
        Some(self.v2()?.us_max_context.get())
    }

    // version ≥ 5:
    pub const fn us_lower_optical_point_size(&self) -> Option<u16> {
        Some(self.v5()?.us_lower_optical_point_size.get())
    }
    pub const fn us_upper_optical_point_size(&self) -> Option<u16> {
        Some(self.v5()?.us_upper_optical_point_size.get())
    }
    pub const fn optical_point_size(&self) -> Option<Range<u16>> {
        Some(self.v5()?.optical_point_size())
    }
}

impl std::fmt::Debug for Os_2<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("OS/2");

        f.field("version", &self.version.get());
        f.field("x_avg_char_width", &self.x_avg_char_width.get());
        f.field("us_weight_class", &self.us_weight_class.get());
        f.field("us_width_class", &self.us_width_class.get());
        f.field("fs_type", fmt_with!("{:?}", self.embedding_rights()));
        f.field("y_subscript_x_size", &self.y_subscript_x_size.get());
        f.field("y_subscript_y_size", &self.y_subscript_y_size.get());
        f.field("y_subscript_x_offset", &self.y_subscript_x_offset.get());
        f.field("y_subscript_y_offset", &self.y_subscript_y_offset.get());
        f.field("y_superscript_x_size", &self.y_superscript_x_size.get());
        f.field("y_superscript_y_size", &self.y_superscript_y_size.get());
        f.field("y_superscript_x_offset", &self.y_superscript_x_offset.get());
        f.field("y_superscript_y_offset", &self.y_superscript_y_offset.get());
        f.field("y_strikeout_size", &self.y_strikeout_size.get());
        f.field("y_strikeout_position", &self.y_strikeout_position.get());
        // TODO: struct for s_family_class?
        f.field("s_family_class", fmt_with!("{:#06X}", self.s_family_class));
        f.field("panose", &self.panose);
        // TODO: impl Debug for UnicodeRanges
        f.field("ul_unicode_range_1", &self.ul_unicode_range_1.get());
        f.field("ul_unicode_range_2", &self.ul_unicode_range_2.get());
        f.field("ul_unicode_range_3", &self.ul_unicode_range_3.get());
        f.field("ul_unicode_range_4", &self.ul_unicode_range_4.get());
        f.field("ach_vend_id", &self.ach_vend_id);
        f.field("fs_selection", fmt_with!("{:?}", self.selection()));

        let ch = self.first_char().unwrap_or('\u{ffff}');
        f.field("us_first_char_index", fmt_with!("{:?} (U+{:04X})", ch, ch as u32));
        let ch = self.last_char().unwrap_or('\u{ffff}');
        f.field("us_last_char_index", fmt_with!("{:?} (U+{:04X})", ch, ch as u32));

        if let Some(v0) = self.v0() {
            f.field("s_typo_ascender", &v0.s_typo_ascender.get());
            f.field("s_typo_descender", &v0.s_typo_descender.get());
            f.field("s_typo_line_gap", &v0.s_typo_line_gap.get());
            f.field("us_win_ascent", &v0.us_win_ascent.get());
            f.field("us_win_descent", &v0.us_win_descent.get());
        }
        if let Some(v1) = self.v1() {
            // TODO: impl Debug for CodePages
            f.field("ul_code_page_range_1", &v1.ul_code_page_range_1.get());
            f.field("ul_code_page_range_2", &v1.ul_code_page_range_2.get());
        }
        if let Some(v2) = self.v2() {
            f.field("sx_height", &v2.sx_height.get());
            f.field("s_cap_height", &v2.s_cap_height.get());

            let ch = v2.default_char();
            f.field("us_default_char", fmt_with!("{:?} (U+{:04X})", ch, ch as u32));
            let ch = v2.break_char();
            f.field("us_break_char", fmt_with!("{:?} (U+{:04X})", ch, ch as u32));

            f.field("us_max_context", &v2.us_max_context.get());
        }
        if let Some(v5) = self.v5() {
            f.field("us_lower_optical_point_size", &v5.us_lower_optical_point_size.get());
            f.field("us_upper_optical_point_size", &v5.us_upper_optical_point_size.get());
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
impl std::fmt::Debug for Os_2V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Os_2 { base: self, len: size_of_val(self) as u32 }.fmt(f)
    }
}
impl std::fmt::Debug for Os_2V5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Os_2 { base: self, len: size_of_val(self) as u32 }.fmt(f)
    }
}
