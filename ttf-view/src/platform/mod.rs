mod encodings;
mod languages;

pub use encodings::*;
pub use languages::*;

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
use define_u16_ids;

define_u16_ids! {
    pub struct PlatformId: u16 {
        Unicode = 0, Macintosh = 1, Iso = 2, Windows = 3, Custom = 4,
    }
}

impl PlatformId {
    pub const fn encoding(self, encoding: u16) -> EncodingId {
        EncodingId::new(self, encoding)
    }
    pub const fn language(self, language: u16) -> LanguageId {
        LanguageId::new(self, language)
    }
}
