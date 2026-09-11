use crate::{
    tables::{Table, TableDirectory, TableError},
    types::{Offset32, Tag, tags, uint16, uint32},
    util::iterator_map,
};
use std::mem::ManuallyDrop;

mod codepoint;
mod glyph_id;

pub use codepoint::*;
pub use glyph_id::*;

pub mod format0;
pub mod format2;
pub mod format4;
pub mod format6;
pub mod format8;

pub mod format10;
pub mod format12;
pub mod format13;
pub mod format14;

#[repr(C)]
pub struct CmapV0 {
    _exhaustive_but_dont_instantiate: (),
    // version ≥ 0:
    pub version: uint16,
    pub num_tables: uint16,
    encoding_records: [EncodingRecordRaw; 0],
}
#[repr(C)]
pub struct EncodingRecordRaw {
    pub platform_id: uint16,
    pub encoding_id: uint16,
    pub subtable_offset: Offset32,
}

impl<'a> Table<'a> for Cmap<'a> {
    const TAG: Tag = tags::cmap;
    fn new_in(dir: &'a TableDirectory) -> Result<Self, TableError> {
        let rec = dir.table_record(Self::TAG).ok_or(TableError::NotFound)?;
        let v0 = rec.raw_as::<CmapV0>().ok_or(TableError::InvalidLen)?;

        let required_len =
            size_of::<CmapV0>() + v0.num_tables.get() as usize * size_of::<EncodingRecordRaw>();
        // Validate that all encoding records are in range
        if (rec.length.get() as usize) < required_len {
            return Err(TableError::InvalidLen);
        }

        for encoding in v0.encodings() {
            // Validate that this subtable's format is in range
            let subtable_offset = encoding.subtable_offset.get() as usize;
            if (rec.length.get() as usize) < subtable_offset + size_of::<uint16>() {
                return Err(TableError::InvalidLen);
            }
            // Calculate this subtable format's header length
            let subtable = encoding.subtable();
            let header_len = match subtable.format() {
                0 | 2 | 4 | 6 => size_of::<ShortMeta>(),
                8 | 10 | 12 | 13 => size_of::<LongMeta>(),
                14 => size_of::<LenOnlyMeta>(),
                _ => 2,
            };
            // Validate that this subtable's header is in range
            if (rec.length.get() as usize) < subtable_offset + header_len {
                return Err(TableError::InvalidLen);
            }
            // Validate that all of this subtable's data is in range
            if (rec.length.get() as usize) < subtable_offset + subtable.length().unwrap() as usize {
                return Err(TableError::InvalidLen);
            }
        }

        Ok(Self { cmap: v0 })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Cmap<'a> {
    cmap: &'a CmapV0,
}

const impl<'a> std::ops::Deref for Cmap<'a> {
    type Target = &'a CmapV0;
    fn deref(&self) -> &Self::Target {
        &self.cmap
    }
}

impl CmapV0 {
    pub const fn encoding_records(&self) -> &[EncodingRecordRaw] {
        let len = self.num_tables.get() as usize;
        unsafe { std::slice::from_raw_parts(self.encoding_records.as_ptr(), len) }
    }
    pub const fn encodings(&self) -> EncodingsIter<'_> {
        EncodingsIter::new(self)
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct EncodingRecord<'a>(&'a CmapV0, &'a EncodingRecordRaw);

const impl<'a> std::ops::Deref for EncodingRecord<'a> {
    type Target = &'a EncodingRecordRaw;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
impl<'a> EncodingRecord<'a> {
    pub const fn subtable(&self) -> &'a CmapSubtable {
        let offset = self.1.subtable_offset.get() as _;
        unsafe { &*std::ptr::from_ref(self.0).cast::<u8>().byte_add(offset).cast() }
    }
}

// TODO: When std::slice::Iter's Clone is constified, make the derive const
pub struct EncodingsIter<'a> {
    cmap: &'a CmapV0,
    inner: std::slice::Iter<'a, EncodingRecordRaw>,
}
impl<'a> EncodingsIter<'a> {
    pub const fn new(cmap: &'a CmapV0) -> Self {
        Self { cmap, inner: cmap.encoding_records().iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, constify as_records()
    pub fn as_records(&self) -> &'a [EncodingRecordRaw] {
        self.inner.as_slice()
    }
}
iterator_map!(EncodingsIter<'a> {
    type Item = EncodingRecord<'a>;
    |this, x| EncodingRecord(this.cmap, x)
});

#[repr(C)]
pub struct CmapSubtable {
    meta: SubtableMeta,
    // : <format-specific data: use meta.*.data to point to>,
}

/// The offsets and sizes of `length` and `language` depend on the subtable's format:
///
/// ```text
///                                        SubtableMeta
///                ┌─────────────────────────────┐─────────────────────────────┐
///                 0    1    2    3    4    5    6
///  ShortMeta     ┌────┬────┬────┬────┬────┬────┐
///  f 0,2,4,6     │ format  │ length  │language │
///                └─────────┴─────────┴─────────┘
///                 0    1    2    3    4    5    6    7    8    9    10   11   12
///  LongMeta      ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
///  f 8,10,12,13  │ format  │reserved │      length       │     language      │
///                └─────────┴─────────┴───────────────────┴───────────────────┘
///                 0    1    2    3    4    5    6
///  LenOnlyMeta   ┌────┬────┬────┬────┬────┬────┐
///  f 14          │ format  │      length       │
///                └─────────┴───────────────────┘
/// ```
#[repr(C)]
union SubtableMeta {
    short: ManuallyDrop<ShortMeta>,
    long: ManuallyDrop<LongMeta>,
    len_only: ManuallyDrop<LenOnlyMeta>,
}

#[repr(C)]
struct ShortMeta {
    format: uint16,
    length: uint16,
    language: uint16,
    data: [u8; 0],
}
#[repr(C)]
struct LongMeta {
    format: uint16,
    reserved: uint16,
    length: uint32,
    language: uint32,
    data: [u8; 0],
}
#[repr(C)]
struct LenOnlyMeta {
    format: uint16,
    length: uint32,
    data: [u8; 0],
}

impl CmapSubtable {
    pub const fn format(&self) -> u16 {
        unsafe { self.meta.short.format.get() }
    }
    pub const fn length(&self) -> Option<u32> {
        Some(match self.format() {
            0 | 2 | 4 | 6 => unsafe { self.meta.short.length.get() as _ },
            8 | 10 | 12 | 13 => unsafe { self.meta.long.length.get() },
            14 => unsafe { self.meta.len_only.length.get() },
            _ => return None,
        })
    }
    pub const fn language(&self) -> Option<u32> {
        Some(match self.format() {
            0 | 2 | 4 | 6 => unsafe { self.meta.short.language.get() as _ },
            8 | 10 | 12 | 13 => unsafe { self.meta.long.language.get() },
            _ => return None,
        })
    }
    const fn data_ptr(&self) -> Option<&u8> {
        Some(match self.format() {
            0 | 2 | 4 | 6 => unsafe { &*self.meta.short.data.as_ptr() },
            8 | 10 | 12 | 13 => unsafe { &*self.meta.long.data.as_ptr() },
            14 => unsafe { &*self.meta.len_only.data.as_ptr() },
            _ => return None,
        })
    }

    pub const fn get_as_bytes(&self) -> Option<&[u8]> {
        match self.format() {
            0 | 2 | 4 | 6 => unsafe {
                let size = self.meta.short.length.get() as usize - size_of::<ShortMeta>();
                Some(std::slice::from_raw_parts(self.meta.short.data.as_ptr(), size))
            },
            8 | 10 | 12 | 13 => unsafe {
                let size = self.meta.long.length.get() as usize - size_of::<LongMeta>();
                Some(std::slice::from_raw_parts(self.meta.long.data.as_ptr(), size))
            },
            14 => unsafe {
                let size = self.meta.len_only.length.get() as usize - size_of::<LenOnlyMeta>();
                Some(std::slice::from_raw_parts(self.meta.len_only.data.as_ptr(), size))
            },
            _ => None,
        }
    }
    pub const fn get_as<T: CmapSubtableTrait>(&self) -> Option<&T> {
        if self.format() == T::FORMAT { Some(unsafe { self.get_as_unchecked() }) } else { None }
    }
    pub const unsafe fn get_as_unchecked<T: CmapSubtableTrait>(&self) -> &T {
        debug_assert!(self.format() == T::FORMAT);
        unsafe { &*std::ptr::from_ref(self.data_ptr().unwrap()).cast() }
    }

    pub const fn as_format0(&self) -> Option<&format0::Format0> {
        self.get_as()
    }
    pub const fn as_format2(&self) -> Option<&format2::Format2> {
        self.get_as()
    }
    pub const fn as_format4(&self) -> Option<&format4::Format4> {
        self.get_as()
    }
    pub const fn as_format6(&self) -> Option<&format6::Format6> {
        self.get_as()
    }
    pub const fn as_format8(&self) -> Option<&format8::Format8> {
        self.get_as()
    }
    pub const fn as_format10(&self) -> Option<&format10::Format10> {
        self.get_as()
    }
    pub const fn as_format12(&self) -> Option<&format12::Format12> {
        self.get_as()
    }
    pub const fn as_format13(&self) -> Option<&format13::Format13> {
        self.get_as()
    }
    pub const fn as_format14(&self) -> Option<&format14::Format14> {
        self.get_as()
    }
}

pub trait CmapSubtableTrait {
    const FORMAT: u16;
}
