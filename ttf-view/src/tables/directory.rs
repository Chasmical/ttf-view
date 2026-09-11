use crate::{
    tables::{Table, TableError},
    types::{Offset32, Tag, tags, uint16, uint32},
    util::iterator_map,
};

#[repr(C)]
pub struct TableDirectory {
    pub sfnt_version: uint32,
    pub num_tables: uint16,
    pub search_range: uint16,
    pub entry_selector: uint16,
    pub range_shift: uint16,
    table_records: [TableRecordRaw; 0],
}
#[repr(C)]
pub struct TableRecordRaw {
    _exhaustive_but_dont_instantiate: (),
    pub table_tag: Tag,
    pub checksum: uint32,
    pub offset: Offset32,
    pub length: uint32,
}

impl TableDirectory {
    pub fn new(bytes: &[u8]) -> Result<&Self, TableError> {
        // Validate that table directory is in range
        if bytes.len() < size_of::<TableDirectory>() {
            return Err(TableError::InvalidLen);
        }
        let dir = unsafe { Self::new_unchecked(bytes) };

        // Validate that all table records are in range
        let required_len = size_of::<TableDirectory>()
            + dir.num_tables.get() as usize * size_of::<TableRecordRaw>();
        if bytes.len() < required_len {
            return Err(TableError::InvalidLen);
        }

        // Validate that every table's data is in range
        for table in dir.table_records_raw() {
            let offset = table.offset.get() as usize + table.length.get() as usize;
            if bytes.len() < offset {
                return Err(TableError::InvalidLen);
            }
        }

        Ok(dir)
    }
    pub const unsafe fn new_unchecked(bytes: &[u8]) -> &Self {
        unsafe { &*bytes.as_ptr().cast() }
    }

    pub const fn directory_as_bytes(&self) -> &[u8] {
        let start = std::ptr::from_ref(self).cast();
        let end = self.table_records_raw().as_ptr_range().end.cast();
        unsafe { std::slice::from_ptr_range(start..end) }
    }

    pub const fn table_records_raw(&self) -> &[TableRecordRaw] {
        let len = self.num_tables.get() as usize;
        unsafe { std::slice::from_raw_parts(self.table_records.as_ptr(), len) }
    }
    pub fn table_record_raw(&self, tag: Tag) -> Option<&TableRecordRaw> {
        self.table_records_raw().iter().find(|x| x.table_tag == tag)
    }

    pub const fn table_records(&self) -> TableRecordsIter<'_> {
        TableRecordsIter::new(self)
    }
    pub fn table_record(&self, tag: Tag) -> Option<TableRecord<'_>> {
        Some(TableRecord(self, self.table_record_raw(tag)?))
    }

    pub fn table<'a, T: Table<'a>>(&'a self) -> Result<T, TableError> {
        T::new_in(self)
    }

    // Note: see src/tables/mod.rs for specific table methods
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct TableRecord<'a>(&'a TableDirectory, &'a TableRecordRaw);

const impl<'a> std::ops::Deref for TableRecord<'a> {
    type Target = &'a TableRecordRaw;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'a> TableRecord<'a> {
    pub const fn table_as_bytes(&self) -> &'a [u8] {
        unsafe {
            let start = std::ptr::from_ref(self.0).cast::<u8>().add(self.offset.get() as _);
            std::slice::from_raw_parts(start, self.length.get() as _)
        }
    }
    pub fn table_as<T: Table<'a>>(&self) -> Result<T, TableError> {
        T::new_in(self.0)
    }

    pub(crate) fn raw_as<T>(&self) -> Option<&'a T> {
        if self.length.get() >= size_of::<T>() as u32 {
            Some(unsafe { &*self.table_as_bytes().as_ptr().cast() })
        } else {
            None
        }
    }

    pub fn calculate_checksum(&self) -> u32 {
        let (uint32s, unpadded) = self.table_as_bytes().as_chunks::<4>();
        let mut sum: u32 = 0;

        for chunk in uint32s {
            sum = sum.wrapping_add(u32::from_be_bytes(*chunk));
        }

        if !unpadded.is_empty() {
            let mut buf = [0; 4];
            buf[..unpadded.len()].copy_from_slice(unpadded);
            sum = sum.wrapping_add(u32::from_be_bytes(buf));
        }

        if self.table_tag == tags::head {
            let checksum_adjustment = u32::from_be_bytes(uint32s[2]);
            sum = sum.wrapping_sub(checksum_adjustment);
        }

        sum
    }
}

// TODO: When std::slice::Iter's Clone is constified, make the derive const
#[derive(Clone)]
pub struct TableRecordsIter<'a> {
    dir: &'a TableDirectory,
    inner: std::slice::Iter<'a, TableRecordRaw>,
}
impl<'a> TableRecordsIter<'a> {
    pub const fn new(dir: &'a TableDirectory) -> Self {
        Self { dir, inner: dir.table_records_raw().iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, constify as_records()
    pub fn as_records(&self) -> &'a [TableRecordRaw] {
        self.inner.as_slice()
    }
}
iterator_map!(TableRecordsIter<'a> {
    type Item = TableRecord<'a>;
    |this, x| TableRecord(this.dir, x)
});

impl std::fmt::Debug for TableDirectory {
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
impl std::fmt::Debug for TableRecordRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TableRecord")
            .field("table_tag", &self.table_tag)
            .field_with("checksum", |f| write!(f, "{:#010X}", self.checksum))
            .field_with("offset", |f| write!(f, "{:#010X}", self.offset))
            .field_with("length", |f| write!(f, "{:#010X}", self.length))
            .finish()
    }
}
impl std::fmt::Debug for TableRecord<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        TableRecordRaw::fmt(self, f)
    }
}
