use crate::types::uint16;

macro_rules! define_id {
    (
        $(
            $(#[$outer:meta])*
            $vis:vis enum $Name:ident {
                $($field:ident = $value:expr),* $(,)?
            }
        )+
    ) => ($(
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        $vis struct $Name([u8; 2]);

        #[allow(non_upper_case_globals)]
        impl $Name {
            pub const fn new(id: u16) -> Self {
                Self(id.to_be_bytes())
            }
            pub const fn from_bytes(bytes: [u8; 2]) -> Self {
                Self(bytes)
            }
            pub const fn get_be(&self) -> uint16 {
                uint16::from_bytes(self.0)
            }
            pub const fn get(&self) -> u16 {
                u16::from_be_bytes(self.0)
            }
            #[allow(clippy::manual_range_patterns)]
            pub const fn is_known(&self) -> bool {
                matches!(self.get(), $($value)|*)
            }

            $( pub const $field: Self = Self::new($value); )*
        }

        impl std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let name = match self.get() {
                    $( $value => stringify!($field), )*
                    _ => "Unknown",
                };
                write!(f, "{} ({})", name, self.get())
            }
        }
    )+);
}

define_id! {
    pub enum PlatformId {
        Unicode = 0, Macintosh = 1, Iso = 2, Windows = 3, Custom = 4,
    }
    pub enum UnicodeEncodingId {
        Unicode1_0 = 0, Unicode1_1 = 1, IsoIec10646 = 2, Unicode2_0BmpOnly = 3, Unicode2_0Full = 4,
        UnicodeVariations = 5, UnicodeFull = 6,
    }
    pub enum MacintoshEncodingId {
        Roman = 0, Japanese = 1, ChineseTraditional = 2, Korean = 3, Arabic = 4, Hebrew = 5,
        Greek = 6, Russian = 7, RSymbol = 8, Devanagari = 9, Gurmukhi = 10, Gujarati = 11,
        Odia = 12, Bangla = 13, Tamil = 14, Telugu = 15, Kannada = 16, Malayalam = 17,
        Sinhalese = 18, Burmese = 19, Khmer = 20, Thai = 21, Laotian = 22, Georgian = 23,
        Armenian = 24, ChineseSimplified = 25, Tibetan = 26, Mongolian = 27, Geez = 28, Slavic = 29,
        Vietnamese = 30, Sindhi = 31, Uninterpreted = 32,
    }
    pub enum IsoEncodingId {
        SevenBitAscii = 0, Iso10646 = 1, Iso8859_1 = 2,
    }
    pub enum WindowsEncodingId {
        Symbol = 0, UnicodeBmp = 1, ShiftJis = 2, Prc = 3, Big5 = 4, Wansung = 5, Johab = 6,
        Reserved7 = 7, Reserved8 = 8, Reserved9 = 9, UnicodeFull = 10,
    }
}

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Encoding {
    platform_id: PlatformId,
    encoding_id: [u8; 2],
}
