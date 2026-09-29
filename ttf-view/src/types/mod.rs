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
mod affine2x3;
mod fixed_point;
mod longdatetime;
mod tag;
mod uint24mod;
mod version16dot16;

pub use affine2x2::*;
pub use affine2x3::*;
pub use fixed_point::*;
pub use longdatetime::*;
pub use tag::*;
pub use uint24mod::*;
pub use version16dot16::*;

macro_rules! define_u16_ids {
    ($(
        $(#[$outer:meta])*
        // u16 is specified explicitly here for clarity at declaration sites
        $vis:vis struct $Name:ident: u16 {
            $( $(#[$inner:meta])* $variant:ident = $value:literal ),* $(,)?
        }
    )*) => ($(
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        $vis struct $Name(u16);

        #[allow(non_upper_case_globals)]
        impl $Name {
            $( $(#[$inner])* pub const $variant: Self = Self($value); )*
        }

        impl $Name {
            pub const fn new(value: u16) -> Self {
                Self(value)
            }
            pub const fn get(self) -> u16 {
                self.0
            }
            #[allow(clippy::manual_range_patterns)]
            pub const fn is_known(&self) -> bool {
                matches!(self.0, $($value)|*)
            }
            pub const fn name(&self) -> Option<&'static str> {
                Some(match self.0 {
                    $( $value => stringify!($variant), )*
                    _ => return None,
                })
            }
        }

        impl std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} ({})", self.get(), self.name().unwrap_or("Unknown"))
            }
        }
    )*);
}
pub(crate) use define_u16_ids;
