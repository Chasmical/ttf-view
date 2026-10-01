use crate::{
    tables::{Table, TableDirectory, TableError},
    types::{BigEndian, Fixed, LongDateTime, Tag, int16, tags, uint16, uint32},
    util::fmt_with,
};

#[repr(C)]
pub struct HeadV0 {
    _exhaustive_but_dont_instantiate: (),
    // version any:
    pub major_version: uint16,
    pub minor_version: uint16,
}
#[repr(C)]
pub struct HeadV1 {
    v0: HeadV0,
    // version = 1.x:
    pub font_revision: BigEndian<Fixed>,
    pub checksum_adjustment: uint32,
    pub magic_number: uint32,
    pub flags: uint16,
    pub units_per_em: uint16,
    pub created: BigEndian<LongDateTime>,
    pub modified: BigEndian<LongDateTime>,
    pub x_min: int16,
    pub y_min: int16,
    pub x_max: int16,
    pub y_max: int16,
    pub mac_style: uint16,
    pub lowest_rec_ppem: uint16,
    pub font_direction_hint: int16,
    pub index_to_loc_format: int16,
    pub glyph_data_format: int16,
}

const impl std::ops::Deref for HeadV1 {
    type Target = HeadV0;
    fn deref(&self) -> &Self::Target {
        &self.v0
    }
}

impl<'a> Table<'a> for Head<'a> {
    const TAG: Tag = tags::head;
    fn new_in(dir: &'a TableDirectory) -> Result<Self, TableError> {
        let rec = dir.table_record(Self::TAG).ok_or(TableError::NotFound)?;
        let v0 = rec.raw_as::<HeadV0>().ok_or(TableError::InvalidLen)?;

        let required_len = match v0.major_version.get() {
            0 => size_of::<HeadV0>() as u32,
            1 => size_of::<HeadV1>() as u32,
            _ => return Err(TableError::UnknownVersion),
        };
        if rec.length.get() < required_len {
            return Err(TableError::InvalidLen);
        }

        Ok(Self { head: v0 })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Head<'a> {
    head: &'a HeadV0,
}

const impl<'a> std::ops::Deref for Head<'a> {
    type Target = &'a HeadV0;
    fn deref(&self) -> &Self::Target {
        &self.head
    }
}

impl<'a> Head<'a> {
    pub const fn v1(&self) -> Option<&'a HeadV1> {
        if self.major_version.get() == 1 {
            Some(unsafe { std::mem::transmute::<&HeadV0, &HeadV1>(self.head) })
        } else {
            None
        }
    }

    // version = 1.x:
    pub const fn font_revision(&self) -> Option<Fixed> {
        Some(self.v1()?.font_revision.get())
    }
    pub const fn checksum_adjustment(&self) -> Option<u32> {
        Some(self.v1()?.checksum_adjustment.get())
    }
    pub const fn magic_number(&self) -> Option<u32> {
        Some(self.v1()?.magic_number.get())
    }
    pub const fn flags(&self) -> Option<u16> {
        Some(self.v1()?.flags.get())
    }
    pub const fn units_per_em(&self) -> Option<u16> {
        Some(self.v1()?.units_per_em.get())
    }
    pub const fn created(&self) -> Option<LongDateTime> {
        Some(self.v1()?.created.get())
    }
    pub const fn modified(&self) -> Option<LongDateTime> {
        Some(self.v1()?.modified.get())
    }
    pub const fn x_min(&self) -> Option<i16> {
        Some(self.v1()?.x_min.get())
    }
    pub const fn y_min(&self) -> Option<i16> {
        Some(self.v1()?.y_min.get())
    }
    pub const fn x_max(&self) -> Option<i16> {
        Some(self.v1()?.x_max.get())
    }
    pub const fn y_max(&self) -> Option<i16> {
        Some(self.v1()?.y_max.get())
    }
    pub const fn mac_style(&self) -> Option<u16> {
        Some(self.v1()?.mac_style.get())
    }
    pub const fn lowest_rec_ppem(&self) -> Option<u16> {
        Some(self.v1()?.lowest_rec_ppem.get())
    }
    pub const fn font_direction_hint(&self) -> Option<i16> {
        Some(self.v1()?.font_direction_hint.get())
    }
    pub const fn index_to_loc_format(&self) -> Option<i16> {
        Some(self.v1()?.index_to_loc_format.get())
    }
    pub const fn glyph_data_format(&self) -> Option<i16> {
        Some(self.v1()?.glyph_data_format.get())
    }
}

impl std::fmt::Debug for Head<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("head");
        f.field("major_version", &self.major_version.get());
        f.field("minor_version", &self.minor_version.get());

        if let Some(v1) = self.v1() {
            f.field("font_revision", &v1.font_revision);
            f.field("checksum_adjustment", fmt_with!("{:#010X}", v1.checksum_adjustment));
            f.field("magic_number", fmt_with!("{:#010X}", v1.magic_number));
            f.field("flags", fmt_with!("{:#017b}", v1.flags));
            f.field("units_per_em", &v1.units_per_em.get());
            f.field("created", fmt_with!("{}", v1.created));
            f.field("modified", fmt_with!("{}", v1.modified));
            f.field("x_min", &v1.x_min.get());
            f.field("y_min", &v1.y_min.get());
            f.field("x_max", &v1.x_max.get());
            f.field("y_max", &v1.y_max.get());
            f.field("mac_style", fmt_with!("{:#09b}", v1.mac_style));
            f.field("lowest_rec_ppem", &v1.lowest_rec_ppem.get());
            f.field("font_direction_hint", &v1.font_direction_hint.get());
            f.field("index_to_loc_format", &v1.index_to_loc_format.get());
            f.field("glyph_data_format", &v1.glyph_data_format.get());
        }
        f.finish()
    }
}

// Monomorphize all fmts to the above fmt for Head<'a>
impl std::fmt::Debug for HeadV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Head { head: self }.fmt(f)
    }
}
impl std::fmt::Debug for HeadV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Head { head: self }.fmt(f)
    }
}
