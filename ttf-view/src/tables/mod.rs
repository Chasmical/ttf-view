use crate::types::Tag;

mod directory;
pub use directory::*;

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod loca;
pub mod maxp;
pub mod name;
pub mod os_2;

impl TableDirectory {
    // Note: Even though these tables are required, we'll still use Option here
    pub fn cmap(&self) -> Result<cmap::Cmap<'_>, TableError> {
        self.table()
    }
    pub fn head(&self) -> Result<head::Head<'_>, TableError> {
        self.table()
    }
    pub fn hhea(&self) -> Result<hhea::Hhea<'_>, TableError> {
        self.table()
    }
    pub fn hmtx(&self) -> Result<hmtx::Hmtx<'_>, TableError> {
        self.table()
    }
    pub fn loca(&self) -> Result<loca::Loca<'_>, TableError> {
        self.table()
    }
    pub fn maxp(&self) -> Result<maxp::Maxp<'_>, TableError> {
        self.table()
    }
    pub fn name(&self) -> Result<name::Name<'_>, TableError> {
        self.table()
    }
    pub fn os_2(&self) -> Result<os_2::Os_2<'_>, TableError> {
        self.table()
    }
}

pub trait Table<'a>: Sized {
    const TAG: Tag;
    fn new_in(dir: &'a TableDirectory) -> Result<Self, TableError>;
}

#[derive(Debug, thiserror::Error)]
#[derive_const(Clone, PartialEq, Eq)]
pub enum TableError {
    #[error("table not found")]
    NotFound,
    #[error("unknown version")]
    UnknownVersion,
    #[error("unknown format")]
    UnknownFormat,
    #[error("dependency '{0}'")]
    Dependency(Tag),
    #[error("dependency: {0}")]
    DependencyError(&'static &'static str),
    #[error("invalid length")]
    InvalidLen,
    #[error("malformed: {0}")]
    Malformed(&'static &'static str),
}
