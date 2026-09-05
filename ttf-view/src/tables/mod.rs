use crate::types::Tag;

mod directory;
pub use directory::*;

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod name;

pub trait Table {
    const TAG: Tag;
    type Handle<'a>: TableHandle<'a>;
    fn in_directory(dir: &TableDirectoryRepr) -> Option<Self::Handle<'_>> {
        Self::Handle::in_directory(dir)
    }
}

pub trait TableHandle<'a>: Sized {
    fn in_directory(dir: &'a TableDirectoryRepr) -> Option<Self>;
}

impl<'a, T: Table> TableHandle<'a> for &'a T {
    fn in_directory(dir: &'a TableDirectoryRepr) -> Option<Self> {
        dir.table_raw::<T>()
    }
}

impl TableDirectoryRepr {
    // Note: Even though these tables are required, we'll still use Option here
    pub fn cmap(&self) -> Option<&cmap::CmapTableRepr> {
        self.table::<cmap::CmapTableRepr>()
    }
    pub fn head(&self) -> Option<&head::HeadTableRepr> {
        self.table::<head::HeadTableRepr>()
    }
    pub fn hhea(&self) -> Option<&hhea::HheaTableRepr> {
        self.table::<hhea::HheaTableRepr>()
    }
    pub fn hmtx(&self) -> Option<hmtx::HmtxTableHandle<'_>> {
        self.table::<hmtx::HmtxTableRepr>()
    }
    pub fn maxp(&self) -> Option<&maxp::MaxpTableRepr> {
        self.table::<maxp::MaxpTableRepr>()
    }
    pub fn name(&self) -> Option<&name::NameTableRepr> {
        self.table::<name::NameTableRepr>()
    }
}
