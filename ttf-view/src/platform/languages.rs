use crate::{platform::PlatformId, tables::name::Name};
use lcid::{LanguageId as Lcid, LcidLookupError};
use std::borrow::Cow;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LanguageId {
    platform: PlatformId,
    language: u16,
}

impl LanguageId {
    pub const fn new(platform: PlatformId, language: u16) -> Self {
        Self { platform, language }
    }
    pub const fn platform(&self) -> PlatformId {
        self.platform
    }
    pub const fn get(&self) -> u16 {
        self.language
    }

    pub fn ietf_tag(&self, table: Option<Name<'_>>) -> Option<Cow<'static, str>> {
        if let Some(idx) = self.language.checked_sub(0x8000) {
            return Some(Cow::Owned(table?.lang_tags().nth(idx as usize)?.string()));
        }
        Some(Cow::Borrowed(match self.platform {
            PlatformId::Macintosh => macintosh_lang_id_to_tag(self.language)?,
            PlatformId::Windows => match <&Lcid>::try_from(self.language as u32) {
                Ok(lcid) => lcid.name,
                Err(LcidLookupError::Reserved(_, tag)) => tag,
                _ => return None,
            },
            _ => return None,
        }))
    }
    pub fn english_name(&self, table: Option<Name<'_>>) -> Option<Cow<'static, str>> {
        if let Some(idx) = self.language.checked_sub(0x8000) {
            let tag = table?.lang_tags().nth(idx as usize)?.string();
            let lcid: &Lcid = tag.as_str().try_into().ok()?;
            return Some(Cow::Borrowed(lcid.english_name));
        }
        Some(Cow::Borrowed(match self.platform {
            PlatformId::Macintosh => macintosh_lang_id_to_name(self.language)?,
            PlatformId::Windows => <&Lcid>::try_from(self.language as u32).ok()?.english_name,
            _ => return None,
        }))
    }

    pub const fn display(self, table: Option<Name<'_>>) -> LanguageDisplay<'_> {
        LanguageDisplay(self, table)
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct LanguageDisplay<'a>(LanguageId, Option<Name<'a>>);

impl std::fmt::Debug for LanguageDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tag = self.0.ietf_tag(self.1).unwrap_or(Cow::Borrowed("und"));
        let name = self.0.english_name(self.1).unwrap_or(Cow::Borrowed("Unknown"));
        write!(f, "{:#06X} ({}: {})", self.0.get(), tag, name)
    }
}

macro_rules! define_macintosh_languages {
    ($( $value:literal, $tag:literal, $name:literal; )*) => {
        fn macintosh_lang_id_to_tag(id: u16) -> Option<&'static str> {
            Some(match id { $( $value => $tag, )* _ => return None })
        }
        fn macintosh_lang_id_to_name(id: u16) -> Option<&'static str> {
            Some(match id { $( $value => $name, )* _ => return None })
        }
    };
}
define_macintosh_languages! {
    0, "en", "English"; 1, "fr", "French"; 2, "de", "German"; 3, "it", "Italian"; 4, "nl", "Dutch";
    5, "sv", "Swedish"; 6, "es", "Spanish"; 7, "da", "Danish"; 8, "pt", "Portuguese";
    9, "no", "Norwegian"; 10, "he", "Hebrew"; 11, "ja", "Japanese"; 12, "ar", "Arabic";
    13, "fi", "Finnish"; 14, "el", "Greek"; 15, "is", "Icelandic"; 16, "mt", "Maltese";
    17, "tr", "Turkish"; 18, "hr", "Croatian"; 19, "zh-Hant", "Chinese (traditional)";
    20, "ur", "Urdu"; 21, "hi", "Hindi"; 22, "th", "Thai"; 23, "ko", "Korean";
    24, "lt", "Lithuanian"; 25, "pl", "Polish"; 26, "hu", "Hungarian"; 27, "et", "Estonian";
    28, "lv", "Latvian"; 29, "se", "Sami"; 30, "fo", "Faroese"; 31, "fa", "Farsi/Persian";
    32, "ru", "Russian"; 33, "zh-Hans", "Chinese (simplified)"; 34, "nl", "Flemish";
    35, "ga", "Irish Gaelic"; 36, "sq", "Albanian"; 37, "ro", "Romanian"; 38, "cs", "Czech";
    39, "sk", "Slovak"; 40, "sl", "Slovenian"; 41, "yi", "Yiddish"; 42, "sr", "Serbian";
    43, "mk", "Macedonian"; 44, "bg", "Bulgarian"; 45, "uk", "Ukrainian"; 46, "be", "Byelorussian";
    47, "uz", "Uzbek"; 48, "kk", "Kazakh"; 49, "az-Cyrl", "Azerbaijani (Cyrillic script)";
    50, "az-Arab", "Azerbaijani (Arabic script)"; 51, "hy", "Armenian"; 52, "ka", "Georgian";
    53, "ro", "Moldavian"; 54, "ky", "Kirghiz"; 55, "tg", "Tajiki"; 56, "tk", "Turkmen";
    57, "mn-Mong", "Mongolian (Mongolian script)"; 58, "mn-Cyrl", "Mongolian (Cyrillic script)";
    59, "ps", "Pashto"; 60, "ku", "Kurdish"; 61, "ks", "Kashmiri"; 62, "sd", "Sindhi";
    63, "bo", "Tibetan"; 64, "ne", "Nepali"; 65, "sa", "Sanskrit"; 66, "mr", "Marathi";
    67, "bn", "Bengali"; 68, "as", "Assamese"; 69, "gu", "Gujarati"; 70, "pa", "Punjabi";
    71, "or", "Oriya"; 72, "ml", "Malayalam"; 73, "kn", "Kannada"; 74, "ta", "Tamil";
    75, "te", "Telugu"; 76, "si", "Sinhalese"; 77, "my", "Burmese"; 78, "km", "Khmer";
    79, "lo", "Lao"; 80, "vi", "Vietnamese"; 81, "id", "Indonesian"; 82, "tl", "Tagalog";
    83, "ms-Latn", "Malay (Roman script)"; 84, "ms-Arab", "Malay (Arabic script)";
    85, "am", "Amharic"; 86, "ti", "Tigrinya"; 87, "om", "Galla"; 88, "so", "Somali";
    89, "sw", "Swahili"; 90, "rw", "Kinyarwanda/Ruanda"; 91, "rn", "Rundi";
    92, "ny", "Nyanja/Chewa"; 93, "mg", "Malagasy"; 94, "eo", "Esperanto"; 128, "cy", "Welsh";
    129, "eu", "Basque"; 130, "ca", "Catalan"; 131, "la", "Latin"; 132, "qu", "Quechua";
    133, "gn", "Guarani"; 134, "ay", "Aymara"; 135, "tt", "Tatar"; 136, "ug", "Uighur";
    137, "dz", "Dzongkha"; 138, "jv-Latn", "Javanese (Roman script)";
    139, "su-Latn", "Sundanese (Roman script)"; 140, "gl", "Galician"; 141, "af", "Afrikaans";
    142, "br", "Breton"; 143, "iu", "Inuktitut"; 144, "gd", "Scottish Gaelic";
    145, "gv", "Manx Gaelic"; 146, "ga", "Irish Gaelic (with dot above)"; 147, "to", "Tongan";
    148, "el", "Greek (polytonic)"; 149, "kl", "Greenlandic";
    150, "az-Latn", "Azerbaijani (Roman script)";
}
