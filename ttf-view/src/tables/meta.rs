use std::bstr::ByteStr;

use crate::{
    tables::Table,
    types::{Offset32, Tag, tags, uint32},
    util::{custom_iterator, fmt_with},
};

use super::{TableDirectory, TableError};

#[repr(C)]
pub struct MetaV0 {
    _exhaustive_but_dont_instantiate: (),
    // version ≥ 0:
    pub version: uint32,
}
#[repr(C)]
pub struct MetaV1 {
    v0: MetaV0,
    // version ≥ 1:
    pub flags: uint32,
    reserved: uint32,
    pub data_maps_count: uint32,
    data_maps: [DataMapRecordRaw; 0],
}

#[repr(C)]
pub struct DataMapRecordRaw {
    _exhaustive_but_dont_instantiate: (),
    pub tag: Tag,
    pub data_offset: Offset32,
    pub data_length: uint32,
}

const impl std::ops::Deref for MetaV1 {
    type Target = MetaV0;
    fn deref(&self) -> &Self::Target {
        &self.v0
    }
}

impl<'a> Table<'a> for Meta<'a> {
    const TAG: Tag = tags::meta;
    fn new_in(dir: &'a TableDirectory) -> Result<Self, TableError> {
        let rec = dir.table_record(Self::TAG).ok_or(TableError::NotFound)?;
        let v0 = rec.raw_as::<MetaV0>().ok_or(TableError::InvalidLen)?;

        if v0.version.get() >= 1 {
            let v1 = rec.raw_as::<MetaV1>().ok_or(TableError::InvalidLen)?;

            let required_len = size_of::<MetaV1>()
                + v1.data_maps_count.get() as usize * size_of::<DataMapRecordRaw>();
            // Validate that all data map records are in range
            if (rec.length.get() as usize) < required_len {
                return Err(TableError::InvalidLen);
            }

            // Validate that all data maps are in range
            for data_map in v1.data_map_records() {
                let (offset, length) = (data_map.data_offset.get(), data_map.data_length.get());
                let offset = offset.checked_add(length).ok_or(TableError::InvalidLen)?;
                if rec.length.get() < offset {
                    return Err(TableError::InvalidLen);
                }
            }
        }

        Ok(Self { meta: v0 })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Meta<'a> {
    meta: &'a MetaV0,
}

const impl<'a> std::ops::Deref for Meta<'a> {
    type Target = &'a MetaV0;
    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

impl<'a> Meta<'a> {
    pub const fn version(&self) -> u32 {
        self.version.get()
    }

    pub const fn v1(&self) -> Option<&'a MetaV1> {
        if self.version() >= 1 {
            Some(unsafe { std::mem::transmute::<&MetaV0, &MetaV1>(self.meta) })
        } else {
            None
        }
    }

    pub const fn data_map_records(&self) -> Option<&'a [DataMapRecordRaw]> {
        Some(self.v1()?.data_map_records())
    }
    pub fn data_map_record(&self, tag: Tag) -> Option<&'a DataMapRecordRaw> {
        self.v1()?.data_map_record(tag)
    }

    pub const fn data_maps(&self) -> DataMapRecordsIter<'a> {
        DataMapRecordsIter::new(*self)
    }
    pub fn data_map(&self, tag: Tag) -> Option<DataMapRecord<'a>> {
        Some(DataMapRecord(*self, self.data_map_record(tag)?))
    }

    pub fn design_languages(&self) -> Option<ScriptLangTags<'_>> {
        Some(ScriptLangTags(self.data_map(tags::dlng)?.utf8().ok()?))
    }
    pub fn supported_languages(&self) -> Option<ScriptLangTags<'_>> {
        Some(ScriptLangTags(self.data_map(tags::slng)?.utf8().ok()?))
    }
}

impl MetaV1 {
    pub const fn data_map_records(&self) -> &[DataMapRecordRaw] {
        let len = self.data_maps_count.get() as usize;
        unsafe { std::slice::from_raw_parts(self.data_maps.as_ptr(), len) }
    }
    pub fn data_map_record(&self, tag: Tag) -> Option<&DataMapRecordRaw> {
        self.data_map_records().iter().find(|x| x.tag == tag)
    }

    pub const fn data_maps(&self) -> DataMapRecordsIter<'_> {
        DataMapRecordsIter::new(Meta { meta: self })
    }
    pub fn data_map(&self, tag: Tag) -> Option<DataMapRecord<'_>> {
        Some(DataMapRecord(Meta { meta: self }, self.data_map_record(tag)?))
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct DataMapRecord<'a>(Meta<'a>, &'a DataMapRecordRaw);

const impl<'a> std::ops::Deref for DataMapRecord<'a> {
    type Target = &'a DataMapRecordRaw;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'a> DataMapRecord<'a> {
    pub const fn bytes(&self) -> &'a [u8] {
        let ptr = std::ptr::from_ref(self.0.meta).cast::<u8>();
        let len = self.data_length.get() as usize;
        unsafe { std::slice::from_raw_parts(ptr.add(self.data_offset.get() as _), len) }
    }
    pub const fn utf8(&self) -> Result<&'a str, std::str::Utf8Error> {
        str::from_utf8(self.bytes())
    }
    pub fn utf8_or_bytes(&self) -> Result<&'a str, &'a ByteStr> {
        let bytes = ByteStr::new(self.bytes());
        str::from_utf8(bytes).or(Err(bytes))
    }
}

// TODO: When std::slice::Iter's Clone is constified, make the derive const
#[derive(Clone)]
pub struct DataMapRecordsIter<'a> {
    table: Meta<'a>,
    inner: std::slice::Iter<'a, DataMapRecordRaw>,
}
impl<'a> DataMapRecordsIter<'a> {
    pub const fn new(table: Meta<'a>) -> Self {
        Self { table, inner: table.data_map_records().unwrap_or(&[]).iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, constify as_records()
    pub fn as_records(&self) -> &'a [DataMapRecordRaw] {
        self.inner.as_slice()
    }
}
custom_iterator!(DataMapRecordsIter<'a> as this {
    type Item = DataMapRecord<'a>;
    map: |x| DataMapRecord(this.table, x);
});

#[derive(Copy)]
#[derive_const(Clone)]
pub struct ScriptLangTags<'a>(&'a str);

impl<'a> ScriptLangTags<'a> {
    pub const fn new(script_lang_tags: &'a str) -> Self {
        Self(script_lang_tags)
    }
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

impl<'a> Iterator for ScriptLangTags<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        Some(match self.0.split_once(',') {
            Some((next, remaining)) => {
                // TODO: When str::trim_prefix is stabilized, use it here.
                self.0 = remaining.strip_prefix(' ').unwrap_or(remaining);
                next
            },
            None => std::mem::take(&mut self.0),
        })
    }
}

impl std::fmt::Debug for Meta<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("meta");
        f.field("version", &self.version.get());

        if let Some(v1) = self.v1() {
            f.field("flags", fmt_with!("{:#010X}", v1.flags));
            if v1.reserved.get() != 0 {
                f.field("reserved", fmt_with!("{:#010X}", v1.reserved));
            }
            f.field("data_maps_count", &v1.data_maps_count.get());
            f.field("data_maps", &v1.data_maps());
        }

        f.finish()
    }
}

// Monomorphize all fmts to the above fmt for Meta<'a>
impl std::fmt::Debug for MetaV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Meta { meta: self }.fmt(f)
    }
}
impl std::fmt::Debug for MetaV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Meta { meta: self }.fmt(f)
    }
}

impl std::fmt::Debug for DataMapRecordsIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl std::fmt::Debug for DataMapRecord<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DataMap")
            .field("tag", &self.tag)
            .field("data_offset", fmt_with!("{:#010X}", self.data_offset))
            .field("data_length", &self.data_length.get())
            .field("data", fmt_with!("{:?}", self.utf8_or_bytes()))
            .finish()
    }
}
