use crate::types::define_u16_ids;

mod encodings;
mod languages;

pub use encodings::*;
pub use languages::*;

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
