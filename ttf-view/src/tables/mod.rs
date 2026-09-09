use crate::types::Tag;

mod directory;
pub use directory::*;

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod name;
pub mod os_2;

pub trait RawTable {
    const TAG: Tag;
}
pub trait Table<'a>: Sized {
    const TAG: Tag;
    fn in_directory(dir: &'a TableDirectoryRepr) -> Option<Self>;
}

impl TableDirectoryRepr {
    // Note: Even though these tables are required, we'll still use Option here
    pub fn cmap(&self) -> Option<cmap::Cmap<'_>> {
        self.table()
    }
    pub fn head(&self) -> Option<head::Head<'_>> {
        self.table()
    }
    pub fn hhea(&self) -> Option<hhea::Hhea<'_>> {
        self.table()
    }
    pub fn hmtx(&self) -> Option<hmtx::Hmtx<'_>> {
        self.table()
    }
    pub fn maxp(&self) -> Option<maxp::Maxp<'_>> {
        self.table()
    }
    pub fn name(&self) -> Option<name::Name<'_>> {
        self.table()
    }
    pub fn os_2(&self) -> Option<os_2::Os_2<'_>> {
        self.table()
    }
}
