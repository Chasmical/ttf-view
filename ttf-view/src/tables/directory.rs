use crate::{
    tables::Table,
    types::{Offset32, Tag, tags, uint16, uint32},
    util::iterator_map,
};

#[repr(C)]
#[non_exhaustive]
pub struct TableDirectoryRepr {
    pub sfnt_version: uint32,
    pub num_tables: uint16,
    pub search_range: uint16,
    pub entry_selector: uint16,
    pub range_shift: uint16,
    table_records: [TableRecordRepr; 0],
}
#[repr(C)]
pub struct TableRecordRepr {
    pub table_tag: Tag,
    pub checksum: uint32,
    pub offset: Offset32,
    pub length: uint32,
}

impl TableDirectoryRepr {
    pub const unsafe fn new_unchecked(bytes: &[u8]) -> &Self {
        unsafe { &*bytes.as_ptr().cast() }
    }

    pub const fn directory_as_bytes(&self) -> &[u8] {
        let start = std::ptr::from_ref(self).cast();
        let end = self.table_records_raw().as_ptr_range().end.cast();
        unsafe { std::slice::from_ptr_range(start..end) }
    }

    pub const fn table_records_raw(&self) -> &[TableRecordRepr] {
        let len = self.num_tables.get() as usize;
        unsafe { std::slice::from_raw_parts(self.table_records.as_ptr(), len) }
    }
    pub fn table_record_raw(&self, tag: Tag) -> Option<&TableRecordRepr> {
        self.table_records_raw().iter().find(|x| x.table_tag == tag)
    }

    pub const fn table_records(&self) -> TableRecordsIter<'_> {
        TableRecordsIter::new(self)
    }
    pub fn table_record(&self, tag: Tag) -> Option<TableRecordHandle<'_>> {
        Some(TableRecordHandle(self, self.table_record_raw(tag)?))
    }

    pub fn table_raw<T: Table>(&self) -> Option<&T> {
        self.table_record(T::TAG)?.table_as()
    }
    pub fn table<T: Table>(&self) -> Option<T::Handle<'_>> {
        T::in_directory(self)
    }

    // Note: see src/tables/mod.rs for specific table methods
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct TableRecordHandle<'a>(&'a TableDirectoryRepr, &'a TableRecordRepr);

const impl std::ops::Deref for TableRecordHandle<'_> {
    type Target = TableRecordRepr;
    fn deref(&self) -> &Self::Target {
        self.1
    }
}

impl<'a> TableRecordHandle<'a> {
    pub const fn table_as_bytes(&self) -> &'a [u8] {
        unsafe {
            let start = std::ptr::from_ref(self.0).cast::<u8>().add(self.offset.get() as _);
            std::slice::from_raw_parts(start, self.length.get() as _)
        }
    }
    pub const fn table_as<T: Table>(&self) -> Option<&'a T> {
        if self.table_tag == T::TAG { Some(unsafe { self.table_as_unchecked() }) } else { None }
    }
    pub const unsafe fn table_as_unchecked<T: Table>(&self) -> &'a T {
        debug_assert!(self.table_tag == T::TAG);
        unsafe { &*self.table_as_bytes().as_ptr().cast() }
    }

    pub fn calculate_checksum(&self) -> u32 {
        let (uint32s, rest) = self.table_as_bytes().as_chunks::<4>();
        let mut sum: u32 = uint32s.iter().map(|x| u32::from_be_bytes(*x)).sum();

        if !rest.is_empty() {
            let mut buf = [0; 4];
            buf[..rest.len()].copy_from_slice(rest);
            sum += u32::from_be_bytes(buf);
        }

        if self.table_tag == tags::head {
            let checksum_adjustment = u32::from_be_bytes(uint32s[2]);
            sum -= checksum_adjustment;
        }

        sum
    }
}

// TODO: When std::slice::Iter's Clone is constified, make the derive const
#[derive(Clone)]
pub struct TableRecordsIter<'a> {
    dir: &'a TableDirectoryRepr,
    inner: std::slice::Iter<'a, TableRecordRepr>,
}
impl<'a> TableRecordsIter<'a> {
    pub const fn new(dir: &'a TableDirectoryRepr) -> Self {
        Self { dir, inner: dir.table_records_raw().iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, constify as_records()
    pub fn as_records(&self) -> &'a [TableRecordRepr] {
        self.inner.as_slice()
    }
}
iterator_map!(TableRecordsIter<'a> {
    type Item = TableRecordHandle<'a>;
    |this, x| TableRecordHandle(this.dir, x)
});

impl std::fmt::Debug for TableDirectoryRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TableDirectory")
            .field_with("sfnt_version", |f| write!(f, "{:#010X}", self.sfnt_version))
            .field("num_tables", &self.num_tables.get())
            .field("search_range", &self.search_range.get())
            .field("entry_selector", &self.entry_selector.get())
            .field("range_shift", &self.range_shift.get())
            .field_with("table_records", |f| {
                let mut list = f.debug_list();

                for table in self.table_records() {
                    list.entry_with(|f| {
                        table.fmt(&mut f.with_options(*f.options().alternate(false)))
                    });
                }
                list.finish()
            })
            .finish()
    }
}
impl std::fmt::Debug for TableRecordRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TableRecord")
            .field("table_tag", &self.table_tag)
            .field_with("checksum", |f| write!(f, "{:#010X}", self.checksum))
            .field_with("offset", |f| write!(f, "{:#010X}", self.offset))
            .field_with("length", |f| write!(f, "{:#010X}", self.length))
            .finish()
    }
}
impl std::fmt::Debug for TableRecordHandle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        TableRecordRepr::fmt(self, f)
    }
}
