use crate::platform::{PlatformId, define_u16_ids};
use std::borrow::Cow;

#[cfg(feature = "non-standard-encodings")]
use encoding_rs::{BIG5, EUC_KR, Encoding as EncodingRs, GB18030, MACINTOSH, SHIFT_JIS};

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncodingId {
    platform: PlatformId,
    encoding: u16,
}

define_u16_ids! {
    pub struct UnicodeEncodingId: u16 {
        Unicode1_0 = 0, Unicode1_1 = 1, IsoIec10646 = 2, Unicode2_0BmpOnly = 3, Unicode2_0Full = 4,
        UnicodeVariations = 5, UnicodeFull = 6,
    }
    pub struct MacintoshEncodingId: u16 {
        Roman = 0, Japanese = 1, ChineseTraditional = 2, Korean = 3, Arabic = 4, Hebrew = 5,
        Greek = 6, Russian = 7, RSymbol = 8, Devanagari = 9, Gurmukhi = 10, Gujarati = 11,
        Odia = 12, Bangla = 13, Tamil = 14, Telugu = 15, Kannada = 16, Malayalam = 17,
        Sinhalese = 18, Burmese = 19, Khmer = 20, Thai = 21, Laotian = 22, Georgian = 23,
        Armenian = 24, ChineseSimplified = 25, Tibetan = 26, Mongolian = 27, Geez = 28,
        Slavic = 29, Vietnamese = 30, Sindhi = 31, Uninterpreted = 32,
    }
    pub struct IsoEncodingId: u16 {
        SevenBitAscii = 0, Iso10646 = 1, Iso8859_1 = 2,
    }
    pub struct WindowsEncodingId: u16 {
        Symbol = 0, UnicodeBmp = 1, ShiftJis = 2, Prc = 3, Big5 = 4, Wansung = 5, Johab = 6,
        UnicodeFull = 10,
    }
}

#[derive(Debug, thiserror::Error)]
#[derive_const(Clone, PartialEq, Eq)]
pub enum EncodingError {
    #[error("unknown platform id")]
    UnknownPlatform,
    #[error("unknown encoding id")]
    UnknownEncoding,
    #[error("string data could not be decoded")]
    MalformedString,
    #[error("this encoding is not implemented")]
    Unimplemented,
}

impl EncodingId {
    pub const fn new(platform: PlatformId, encoding: u16) -> Self {
        Self { platform, encoding }
    }
    pub const fn platform(&self) -> PlatformId {
        self.platform
    }
    pub const fn get(&self) -> u16 {
        self.encoding
    }

    pub const fn is_known(&self) -> bool {
        match self.platform {
            PlatformId::Unicode => UnicodeEncodingId::new(self.encoding).is_known(),
            PlatformId::Macintosh => MacintoshEncodingId::new(self.encoding).is_known(),
            PlatformId::Iso => IsoEncodingId::new(self.encoding).is_known(),
            PlatformId::Windows => WindowsEncodingId::new(self.encoding).is_known(),
            PlatformId::Custom => self.encoding <= 255,
            _ => false,
        }
    }
    pub fn name(&self) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed(match self.platform {
            PlatformId::Unicode => UnicodeEncodingId::new(self.encoding).name()?,
            PlatformId::Macintosh => MacintoshEncodingId::new(self.encoding).name()?,
            PlatformId::Iso => IsoEncodingId::new(self.encoding).name()?,
            PlatformId::Windows => WindowsEncodingId::new(self.encoding).name()?,
            PlatformId::Custom => {
                let charset = u8::try_from(self.encoding).ok()?;
                return Some(Cow::Owned(format!("Charset {}", charset)));
            },
            _ => return None,
        }))
    }

    pub fn decode(&self, bytes: &[u8]) -> Result<String, EncodingError> {
        use EncodingError::*;

        fn decode_utf16be(bytes: &[u8]) -> Result<String, EncodingError> {
            String::from_utf16be(bytes).or(Err(MalformedString))
        }

        #[cfg(feature = "non-standard-encodings")]
        fn decode_with(bytes: &[u8], enc: &'static EncodingRs) -> Result<String, EncodingError> {
            let decoded = enc.decode_without_bom_handling_and_without_replacement(bytes);
            decoded.ok_or(MalformedString).map(|x| x.into_owned())
        }

        match self.platform {
            PlatformId::Unicode => {
                let enc = UnicodeEncodingId::new(self.encoding);
                // All Unicode encodings use UTF-16BE
                if enc.is_known() { decode_utf16be(bytes) } else { Err(UnknownEncoding) }
            },

            PlatformId::Macintosh => {
                use MacintoshEncodingId as Id;

                match Id::new(self.encoding) {
                    // Try to use encoding_rs if it's available (it's vectorized)
                    #[cfg(feature = "non-standard-encodings")]
                    Id::Roman => decode_with(bytes, MACINTOSH), // Mac OS Roman
                    // Otherwise, rely on a small but inefficient bundled implementation
                    #[cfg(not(feature = "non-standard-encodings"))]
                    Id::Roman => Ok(bundled_macos_roman::decode(bytes)), // Mac OS Roman

                    enc => Err(if enc.is_known() { Unimplemented } else { UnknownEncoding }),
                }
            },

            PlatformId::Iso => {
                let enc = IsoEncodingId::new(self.encoding);
                Err(if enc.is_known() { Unimplemented } else { UnknownEncoding })
            },

            PlatformId::Windows => {
                use WindowsEncodingId as Id;

                match Id::new(self.encoding) {
                    Id::Symbol | Id::UnicodeBmp | Id::UnicodeFull => decode_utf16be(bytes),

                    #[cfg(feature = "non-standard-encodings")]
                    Id::ShiftJis => decode_with(bytes, SHIFT_JIS), // ShiftJIS / CP 932
                    #[cfg(feature = "non-standard-encodings")]
                    Id::Prc => decode_with(bytes, GB18030), // PRC / CP 936 or 54936?
                    #[cfg(feature = "non-standard-encodings")]
                    Id::Big5 => decode_with(bytes, BIG5), // Big5 / CP 950
                    #[cfg(feature = "non-standard-encodings")]
                    Id::Wansung => decode_with(bytes, EUC_KR), // Wansung / EUC-KR / CP 949

                    // #[cfg(feature = "non-standard-encodings")]
                    // Id::Johab => , // Johab / KS C 5601-1992 / CP 1361
                    enc => Err(if enc.is_known() { Unimplemented } else { UnknownEncoding }),
                }
            },

            PlatformId::Custom => {
                Err(if self.encoding <= 255 { Unimplemented } else { UnknownEncoding })
            },

            _ => Err(UnknownPlatform),
        }
    }
}

impl std::fmt::Debug for EncodingId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.get(), self.name().unwrap_or(Cow::Borrowed("Unknown")))
    }
}

const impl From<UnicodeEncodingId> for EncodingId {
    fn from(value: UnicodeEncodingId) -> Self {
        Self { platform: PlatformId::Unicode, encoding: value.get() }
    }
}
const impl From<MacintoshEncodingId> for EncodingId {
    fn from(value: MacintoshEncodingId) -> Self {
        Self { platform: PlatformId::Macintosh, encoding: value.get() }
    }
}
const impl From<IsoEncodingId> for EncodingId {
    fn from(value: IsoEncodingId) -> Self {
        Self { platform: PlatformId::Iso, encoding: value.get() }
    }
}
const impl From<WindowsEncodingId> for EncodingId {
    fn from(value: WindowsEncodingId) -> Self {
        Self { platform: PlatformId::Windows, encoding: value.get() }
    }
}

#[cfg(not(feature = "non-standard-encodings"))]
mod bundled_macos_roman {
    pub fn decode(bytes: &[u8]) -> String {
        bytes.iter().copied().map(map_macos_roman).collect()
    }
    fn map_macos_roman(byte: u8) -> char {
        if byte < 0x80 {
            byte as char
        } else {
            unsafe {
                char::from_u32_unchecked(*MACOS_ROMAN.get_unchecked((byte - 0x80) as usize) as u32)
            }
        }
    }

    const MACOS_ROMAN: [u16; 128] = [
        196, 197, 199, 201, 209, 214, 220, 225, 224, 226, 228, 227, 229, 231, 233, 232, 234, 235,
        237, 236, 238, 239, 241, 243, 242, 244, 246, 245, 250, 249, 251, 252, 8224, 176, 162, 163,
        167, 8226, 182, 223, 174, 169, 8482, 180, 168, 8800, 198, 216, 8734, 177, 8804, 8805, 165,
        181, 8706, 8721, 8719, 960, 8747, 170, 186, 937, 230, 248, 191, 161, 172, 8730, 402, 8776,
        8710, 171, 187, 8230, 160, 192, 195, 213, 338, 339, 8211, 8212, 8220, 8221, 8216, 8217,
        247, 9674, 255, 376, 8260, 8364, 8249, 8250, 64257, 64258, 8225, 183, 8218, 8222, 8240,
        194, 202, 193, 203, 200, 205, 206, 207, 204, 211, 212, 63743, 210, 218, 219, 217, 305, 710,
        732, 175, 728, 729, 730, 184, 733, 731, 711,
    ];
}
