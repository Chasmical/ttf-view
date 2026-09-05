use crate::{
    platform::{EncodingError, EncodingId, PlatformId},
    types::{Offset16, Tag, tags, uint16},
    util::iterator_map,
};
use std::{borrow::Cow, bstr::ByteStr};

#[repr(C)]
#[non_exhaustive]
pub struct NameTableRepr {
    // version ≥ 0:
    pub version: uint16,
    pub count: uint16,
    pub storage_offset: Offset16,
    name_records: [NameRecordRepr; 0],
    // version ≥ 1:
    // : lang_tag_count: uint16,
    // : lang_tag_records: [LangTagRecordRepr; lang_tag_count],
}
#[repr(C)]
pub struct NameRecordRepr {
    pub platform_id: uint16,
    pub encoding_id: uint16,
    pub language_id: uint16,
    pub name_id: uint16,
    pub length: uint16,
    pub string_offset: Offset16,
}
#[repr(C)]
pub struct LangTagRecordRepr {
    pub length: uint16,
    pub lang_tag_offset: Offset16,
}

impl super::Table for NameTableRepr {
    const TAG: Tag = tags::name;
    type Handle<'a> = &'a Self;
}

impl NameTableRepr {
    pub const fn name_records(&self) -> &[NameRecordRepr] {
        unsafe { std::slice::from_raw_parts(self.name_records.as_ptr(), self.count.get() as _) }
    }
    pub const fn names(&self) -> NamesIter<'_> {
        NamesIter::new(self)
    }

    // Note: I decided not to include lang_tag_count() getter here, because we've already got
    // not only a better higher-level API - lang_tags(), but also just lang_tag_records().

    pub const fn lang_tag_records(&self) -> &[LangTagRecordRepr] {
        if self.version.get() == 0 {
            return &[];
        }
        let len_ptr = self.name_records().as_ptr_range().end.cast::<uint16>();
        unsafe { std::slice::from_raw_parts(len_ptr.add(1).cast(), (*len_ptr).get() as _) }
    }
    pub const fn lang_tags(&self) -> LangTagsIter<'_> {
        LangTagsIter::new(self)
    }

    pub const fn string_storage(&self) -> &StringStorage {
        let offset = self.storage_offset.get() as usize;
        unsafe { &*std::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

#[non_exhaustive]
pub struct StringStorage {}

impl StringStorage {
    pub const fn as_ptr(&self) -> *const u8 {
        std::ptr::from_ref(self).cast()
    }
    pub(crate) const unsafe fn get(&self, offset: u16, length: u16) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_ptr().add(offset as _), length as _) }
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct NameHandle<'a>(&'a NameTableRepr, &'a NameRecordRepr);

#[derive(Copy)]
#[derive_const(Clone)]
pub struct LangTagHandle<'a>(&'a NameTableRepr, &'a LangTagRecordRepr);

const impl std::ops::Deref for NameHandle<'_> {
    type Target = NameRecordRepr;
    fn deref(&self) -> &Self::Target {
        self.1
    }
}
const impl std::ops::Deref for LangTagHandle<'_> {
    type Target = LangTagRecordRepr;
    fn deref(&self) -> &Self::Target {
        self.1
    }
}

impl<'a> NameHandle<'a> {
    pub const fn bytes(&self) -> &'a [u8] {
        unsafe { self.0.string_storage().get(self.1.string_offset.get(), self.1.length.get()) }
    }
    pub fn string(&self) -> Result<String, EncodingError> {
        let encoding = EncodingId::new(self.1.platform_id.get(), self.1.encoding_id.get())?;
        encoding.decode_utf16be(self.bytes())
    }
}

impl<'a> LangTagHandle<'a> {
    pub const fn bytes(&self) -> &'a [u8] {
        unsafe { self.0.string_storage().get(self.1.lang_tag_offset.get(), self.1.length.get()) }
    }
    pub fn string(&self) -> String {
        // Note: LangTags are always encoded in UTF-16BE.
        String::from_utf16be_lossy(self.bytes())
    }
}

// TODO: When std::slice::Iter's Clone is constified, make the derive const
#[derive(Clone)]
pub struct NamesIter<'a> {
    table: &'a NameTableRepr,
    inner: std::slice::Iter<'a, NameRecordRepr>,
}
impl<'a> NamesIter<'a> {
    pub const fn new(table: &'a NameTableRepr) -> Self {
        Self { table, inner: table.name_records().iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, constify as_records()
    pub fn as_records(&self) -> &'a [NameRecordRepr] {
        self.inner.as_slice()
    }
}
iterator_map!(NamesIter<'a> {
    type Item = NameHandle<'a>;
    |this, x| NameHandle(this.table, x)
});

// TODO: When std::slice::Iter's Clone is constified, make the derive const
#[derive(Clone)]
pub struct LangTagsIter<'a> {
    table: &'a NameTableRepr,
    inner: std::slice::Iter<'a, LangTagRecordRepr>,
}
impl<'a> LangTagsIter<'a> {
    pub const fn new(table: &'a NameTableRepr) -> Self {
        Self { table, inner: table.lang_tag_records().iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, constify as_records()
    pub fn as_records(&self) -> &'a [LangTagRecordRepr] {
        self.inner.as_slice()
    }
}
iterator_map!(LangTagsIter<'a> {
    type Item = LangTagHandle<'a>;
    |this, x| LangTagHandle(this.table, x)
});

impl std::fmt::Debug for NameTableRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut builder = f.debug_struct("NameTable");

        builder
            .field("version", &self.version.get())
            .field("count", &self.count.get())
            .field_with("storage_offset", |f| write!(f, "{:#06X}", self.storage_offset))
            .field_with("name_records", |f| f.debug_list().entries(self.names()).finish());

        if self.version.get() != 0 {
            builder
                .field("lang_tag_count", &self.lang_tag_records().len())
                .field_with("lang_tag_records", |f| {
                    f.debug_list().entries(self.lang_tags()).finish()
                });
        }

        builder.finish()
    }
}

impl std::fmt::Debug for NameHandle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self(name, rec) = *self;

        let value = self.string().map_err(|err| (err, ByteStr::new(self.bytes())));

        let plat_id = PlatformId::new(rec.platform_id.get());
        let plat_name = plat_id.map_or("Unknown", |x| x.name());

        let enc_id = plat_id.and_then(|x| x.encoding(rec.encoding_id.get()));
        let enc_name = enc_id.map_or(Cow::Borrowed("Unknown"), |x| x.name());

        let lang_id = plat_id.and_then(|x| x.language(rec.language_id.get()));
        let lang_name = format!(
            "{}: {}",
            lang_id.and_then(|x| x.tag(Some(name))).unwrap_or(Cow::Borrowed("und")),
            lang_id.and_then(|x| x.english_name(Some(name))).unwrap_or(Cow::Borrowed("Unknown")),
        );

        f.debug_struct("NameRecord")
            .field_with("platform_id", |f| write!(f, "{} ({})", rec.platform_id, plat_name))
            .field_with("encoding_id", |f| write!(f, "{} ({})", rec.encoding_id, enc_name))
            .field_with("language_id", |f| write!(f, "{:#06X} ({})", rec.language_id, lang_name))
            // TODO: Parse name_id and display its name
            .field("name_id", &rec.name_id.get())
            .field("length", &rec.length.get())
            .field_with("string_offset", |f| write!(f, "{:#06X}", rec.string_offset))
            .field_with("value", |f| {
                let mut f = f.with_options(*f.options().alternate(false));
                value.fmt(&mut f)
            })
            .finish()
    }
}

impl std::fmt::Debug for LangTagHandle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self(_, rec) = *self;

        f.debug_struct("LangTagRecord")
            .field("length", &rec.length.get())
            .field_with("lang_tag_offset", |f| write!(f, "{:#06X}", rec.lang_tag_offset))
            .field_with("value", |f| {
                let mut f = f.with_options(*f.options().alternate(false));
                self.string().fmt(&mut f)
            })
            .finish()
    }
}
