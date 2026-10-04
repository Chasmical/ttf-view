use crate::types::Tag;

mod directory;
pub use directory::*;

pub mod cmap;
pub mod glyf;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod loca;
pub mod maxp;
pub mod meta;
pub mod name;
pub mod os_2;

impl<'a> TableDirectory<'a> {
    // Note: Even though these tables are required, we'll still use Option here
    pub fn cmap(&self) -> Result<cmap::Cmap<'a>, TableError> {
        self.table()
    }
    pub fn glyf(&self) -> Result<glyf::Glyf<'a>, TableError> {
        self.table()
    }
    pub fn head(&self) -> Result<head::Head<'a>, TableError> {
        self.table()
    }
    pub fn hhea(&self) -> Result<hhea::Hhea<'a>, TableError> {
        self.table()
    }
    pub fn hmtx(&self) -> Result<hmtx::Hmtx<'a>, TableError> {
        self.table()
    }
    pub fn loca(&self) -> Result<loca::Loca<'a>, TableError> {
        self.table()
    }
    pub fn maxp(&self) -> Result<maxp::Maxp<'a>, TableError> {
        self.table()
    }
    pub fn meta(&self) -> Result<meta::Meta<'a>, TableError> {
        self.table()
    }
    pub fn name(&self) -> Result<name::Name<'a>, TableError> {
        self.table()
    }
    pub fn os_2(&self) -> Result<os_2::Os_2<'a>, TableError> {
        self.table()
    }
}

pub trait Table<'a>: Sized {
    const TAG: Tag;
    fn new_in(dir: TableDirectory<'a>) -> Result<Self, TableError>;
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
