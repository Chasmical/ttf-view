#![allow(non_camel_case_types)]
use zerocopy::network_endian::{I16, I32, I64, U16, U32, U64};

mod big_endian;
pub use big_endian::*;

pub type int16 = I16;
pub type int32 = I32;
pub type int64 = I64;
pub type uint16 = U16;
pub type uint24 = BigEndian<u24>;
pub type uint32 = U32;
pub type uint64 = U64;

// These distinctions aren't really necessary, but we'll keep it for better understanding of values
pub type FWORD = int16;
pub type UFWORD = uint16;

pub type Offset8 = u8;
pub type Offset16 = uint16;
pub type Offset24 = uint24;
pub type Offset32 = uint32;

mod affine2x2;
mod fixed_point;
mod longdatetime;
mod tag;
mod uint24mod;
mod version16dot16;

pub use affine2x2::*;
pub use fixed_point::*;
pub use longdatetime::*;
pub use tag::*;
pub use uint24mod::*;
pub use version16dot16::*;
