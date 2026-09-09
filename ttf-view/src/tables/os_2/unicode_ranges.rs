#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct UnicodeRanges(u128);

impl UnicodeRanges {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_bits(bits: u128) -> Self {
        Self(bits)
    }
    pub const fn bits(&self) -> u128 {
        self.0
    }

    pub const fn from_parts(ul1: u32, ul2: u32, ul3: u32, ul4: u32) -> Self {
        Self::from_bits(
            (ul1 as u128) | ((ul2 as u128) << 32) | ((ul3 as u128) << 64) | ((ul4 as u128) << 96),
        )
    }
    pub const fn into_parts(self) -> (u32, u32, u32, u32) {
        (
            self.bits() as u32,
            (self.bits() >> 32) as u32,
            (self.bits() >> 64) as u32,
            (self.bits() >> 96) as u32,
        )
    }

    pub fn from_chars(iter: impl IntoIterator<Item = char>) -> Self {
        iter.into_iter().fold(Self::empty(), |mut acc, ch| {
            if let Some(range) = UnicodeRange::from_char(ch) {
                acc |= range;
            }
            if matches!(ch as u32, 0x10000..=0x10FFFF) {
                acc |= Self::from_bits(1 << 57);
            }
            acc
        })
    }

    // TODO: iter UnicodeRanges
}

const impl From<UnicodeRange> for UnicodeRanges {
    fn from(value: UnicodeRange) -> Self {
        Self(1 << value.bit_index())
    }
}
const impl std::ops::BitOr for UnicodeRanges {
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
    type Output = Self;
}
const impl std::ops::BitOrAssign for UnicodeRanges {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0);
    }
}
const impl std::ops::BitOr<UnicodeRange> for UnicodeRanges {
    fn bitor(self, rhs: UnicodeRange) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
    type Output = Self;
}
const impl std::ops::BitOrAssign<UnicodeRange> for UnicodeRanges {
    fn bitor_assign(&mut self, rhs: UnicodeRange) {
        self.bitor_assign(Self::from(rhs));
    }
}

macro_rules! define_unicode_ranges {
    ($(
        $bit_index:literal, $from:literal ..= $to:literal => $field:ident $((v $version:literal))?;
    )*) => {
        #[repr(u8)]
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum UnicodeRange {
            $(#[doc(hidden)] $field,)*
            Non_Plane_0,
        }

        impl UnicodeRange {
            #[allow(unreachable_patterns)]
            pub const fn from_char(ch: char) -> Option<Self> {
                Some(match ch as u32 {
                    $($from..=$to => Self::$field,)*
                    0x10000..=0x10FFFF => Self::Non_Plane_0,
                    _ => return None,
                })
            }
            pub const fn bit_index(&self) -> u8 {
                match self {
                    $(Self::$field => $bit_index,)*
                    Self::Non_Plane_0 => 57,
                }
            }
            pub const fn name(&self) -> &'static str {
                match self {
                    $(Self::$field => stringify!($field),)*
                    Self::Non_Plane_0 => "Non_Plane_0",
                }
            }
            pub const fn range(&self) -> Option<(u32, u32)> {
                let (from, to) = match self {
                    $(Self::$field => ($from, $to),)*
                    Self::Non_Plane_0 => (0, 0),
                };
                if to != 0 { Some((from, to)) } else { None }
            }
            pub const fn version(&self) -> u8 {
                match self {
                    $(Self::$field => define_unicode_ranges!(@since $($version)?),)*
                    Self::Non_Plane_0 => 2,
                }
            }
        }
    };
    (@since) => (1);
    (@since $version:literal) => ($version);
}

define_unicode_ranges! {
    // OS/2 v2 = OT v1.3
    // OS/2 v3 = OT v1.4
    // OS/2 v4 = OT v1.5

    0, 0x0000..=0x007F => Basic_Latin;
    1, 0x0080..=0x00FF => Latin_1_Supplement;
    2, 0x0100..=0x017F => Latin_Extended_A;
    3, 0x0180..=0x024F => Latin_Extended_B;
    4, 0x0250..=0x02AF => IPA_Extensions;
    4, 0x1D00..=0x1D7F => Phonetic_Extensions (v 4);
    4, 0x1D80..=0x1DBF => Phonetic_Extensions_Supplement (v 4);
    5, 0x02B0..=0x02FF => Spacing_Modifier_Letters;
    5, 0xA700..=0xA71F => Modifier_Tone_Letters (v 4);
    6, 0x0300..=0x036F => Combining_Diacritical_Marks;
    6, 0x1DC0..=0x1DFF => Combining_Diacritical_Marks_Supplement (v 4);
    7, 0x0370..=0x03FF => Greek_and_Coptic; // Note: used to be non-well-defined as "Basic Greek" in v1 (fixed in v2)
    8, 0x2C80..=0x2CFF => Coptic (v 4); // Note: used to be non-well-defined as "Greek Symbols and Coptic" in v1 (fixed in v2)
    9, 0x0400..=0x04FF => Cyrillic;
    9, 0x0500..=0x052F => Cyrillic_Supplement (v 3);
    9, 0x2DE0..=0x2DFF => Cyrillic_Extended_A (v 4);
    9, 0xA640..=0xA69F => Cyrillic_Extended_B (v 4);
    10, 0x0530..=0x058F => Armenian;
    11, 0x0590..=0x05FF => Hebrew; // Note: used to be non-well-defined as "Basic Hebrew" in v1 (fixed in v2)
    12, 0xA500..=0xA63F => Vai (v 4); // Note: used to be non-well-defined as "Hebrew Extended" in v1 (fixed in v2)
    13, 0x0600..=0x06FF => Arabic; // Note: used to be non-well-defined as "Basic Arabic" in v1 (fixed in v2)
    13, 0x0750..=0x077F => Arabic_Supplement (v 4);
    14, 0x07C0..=0x07FF => NKo (v 4); // Note: used to be non-well-defined as "Arabic Extended" in v1 (fixed in v2)
    15, 0x0900..=0x097F => Devanagari;
    16, 0x0980..=0x09FF => Bangla;
    17, 0x0A00..=0x0A7F => Gurmukhi;
    18, 0x0A80..=0x0AFF => Gujarati;
    19, 0x0B00..=0x0B7F => Odia;
    20, 0x0B80..=0x0BFF => Tamil;
    21, 0x0C00..=0x0C7F => Telugu;
    22, 0x0C80..=0x0CFF => Kannada;
    23, 0x0D00..=0x0D7F => Malayalam;
    24, 0x0E00..=0x0E7F => Thai;
    25, 0x0E80..=0x0EFF => Lao;
    26, 0x10A0..=0x10FF => Georgian; // Note: used to be non-well-defined as "Basic Georgian" in v1 (fixed in v2)
    26, 0x2D00..=0x2D2F => Georgian_Supplement (v 4);
    27, 0x1B00..=0x1B7F => Balinese (v 4); // Note: used to be non-well-defined as "Georgian Extended" in v1 (fixed in v2)
    28, 0x1100..=0x11FF => Hangul_Jamo;
    29, 0x1E00..=0x1EFF => Latin_Extended_Additional;
    29, 0x2C60..=0x2C7F => Latin_Extended_C (v 4);
    29, 0xA720..=0xA7FF => Latin_Extended_D (v 4);
    30, 0x1F00..=0x1FFF => Greek_Extended;
    31, 0x2000..=0x206F => General_Punctuation;
    31, 0x2E00..=0x2E7F => Supplemental_Punctuation (v 4);
    32, 0x2070..=0x209F => Superscripts_And_Subscripts;
    33, 0x20A0..=0x20CF => Currency_Symbols;
    34, 0x20D0..=0x20FF => Combining_Diacritical_Marks_For_Symbols;
    35, 0x2100..=0x214F => Letterlike_Symbols;
    36, 0x2150..=0x218F => Number_Forms;
    37, 0x2190..=0x21FF => Arrows;
    37, 0x27F0..=0x27FF => Supplemental_Arrows_A (v 3);
    37, 0x2900..=0x297F => Supplemental_Arrows_B (v 3);
    37, 0x2B00..=0x2BFF => Miscellaneous_Symbols_and_Arrows (v 4);
    38, 0x2200..=0x22FF => Mathematical_Operators;
    38, 0x2A00..=0x2AFF => Supplemental_Mathematical_Operators (v 3);
    38, 0x27C0..=0x27EF => Miscellaneous_Mathematical_Symbols_A (v 3);
    38, 0x2980..=0x29FF => Miscellaneous_Mathematical_Symbols_B (v 3);
    39, 0x2300..=0x23FF => Miscellaneous_Technical;
    40, 0x2400..=0x243F => Control_Pictures;
    41, 0x2440..=0x245F => Optical_Character_Recognition;
    42, 0x2460..=0x24FF => Enclosed_Alphanumerics;
    43, 0x2500..=0x257F => Box_Drawing;
    44, 0x2580..=0x259F => Block_Elements;
    45, 0x25A0..=0x25FF => Geometric_Shapes;
    46, 0x2600..=0x26FF => Miscellaneous_Symbols;
    47, 0x2700..=0x27BF => Dingbats;
    48, 0x3000..=0x303F => CJK_Symbols_And_Punctuation;
    49, 0x3040..=0x309F => Hiragana;
    50, 0x30A0..=0x30FF => Katakana;
    50, 0x31F0..=0x31FF => Katakana_Phonetic_Extensions (v 3);
    51, 0x3100..=0x312F => Bopomofo;
    51, 0x31A0..=0x31BF => Bopomofo_Extended (v 2);
    52, 0x3130..=0x318F => Hangul_Compatibility_Jamo;
    53, 0xA840..=0xA87F => Phags_pa (v 4); // Note: Used to be non-well-defined as "CJK Miscellaneous" in v1/v2 (fixed in v3)
    54, 0x3200..=0x32FF => Enclosed_CJK_Letters_And_Months;
    55, 0x3300..=0x33FF => CJK_Compatibility;
    56, 0xAC00..=0xD7AF => Hangul_Syllables;
    // Note: Non_Plane_0 is handled separately in UnicodeRanges
    // TODO: Maybe we should just ignore it? And add/remove as needed in UnicodeRanges?
    // 57, 0x10000..=0x10FFFF => Non_Plane_0 (v 2);
    58, 0x10900..=0x1091F => Phoenician (v 4);
    59, 0x4E00..=0x9FFF => CJK_Unified_Ideographs;
    59, 0x2E80..=0x2EFF => CJK_Radicals_Supplement (v 2);
    59, 0x2F00..=0x2FDF => Kangxi_Radicals (v 2);
    59, 0x2FF0..=0x2FFF => Ideographic_Description_Characters (v 2);
    59, 0x3400..=0x4DBF => CJK_Unified_Ideographs_Extension_A (v 2);
    59, 0x20000..=0x2A6DF => CJK_Unified_Ideographs_Extension_B (v 3);
    59, 0x3190..=0x319F => Kanbun (v 3);
    60, 0xE000..=0xF8FF => Private_Use_Area_Plane_0;
    61, 0x31C0..=0x31EF => CJK_Strokes (v 4);
    61, 0xF900..=0xFAFF => CJK_Compatibility_Ideographs;
    61, 0x2F800..=0x2FA1F => CJK_Compatibility_Ideographs_Supplement (v 3);
    62, 0xFB00..=0xFB4F => Alphabetic_Presentation_Forms;
    63, 0xFB50..=0xFDFF => Arabic_Presentation_Forms_A;
    64, 0xFE20..=0xFE2F => Combining_Half_Marks;
    65, 0xFE10..=0xFE1F => Vertical_Forms (v 4);
    65, 0xFE30..=0xFE4F => CJK_Compatibility_Forms;
    66, 0xFE50..=0xFE6F => Small_Form_Variants;
    67, 0xFE70..=0xFEFF => Arabic_Presentation_Forms_B;
    68, 0xFF00..=0xFFEF => Halfwidth_And_Fullwidth_Forms;
    69, 0xFFF0..=0xFFFF => Specials;
    70, 0x0F00..=0x0FFF => Tibetan (v 2);
    71, 0x0700..=0x074F => Syriac (v 2);
    72, 0x0780..=0x07BF => Thaana (v 2);
    73, 0x0D80..=0x0DFF => Sinhala (v 2);
    74, 0x1000..=0x109F => Myanmar (v 2);
    75, 0x1200..=0x137F => Ethiopic (v 2);
    75, 0x1380..=0x139F => Ethiopic_Supplement (v 4);
    75, 0x2D80..=0x2DDF => Ethiopic_Extended (v 4);
    76, 0x13A0..=0x13FF => Cherokee (v 2);
    77, 0x1400..=0x167F => Unified_Canadian_Aboriginal_Syllabics (v 2);
    78, 0x1680..=0x169F => Ogham (v 2);
    79, 0x16A0..=0x16FF => Runic (v 2);
    80, 0x1780..=0x17FF => Khmer (v 2);
    80, 0x19E0..=0x19FF => Khmer_Symbols (v 4);
    81, 0x1800..=0x18AF => Mongolian (v 2);
    82, 0x2800..=0x28FF => Braille_Patterns (v 2);
    83, 0xA000..=0xA48F => Yi_Syllables (v 2);
    83, 0xA490..=0xA4CF => Yi_Radicals (v 2);
    84, 0x1700..=0x171F => Tagalog (v 3);
    84, 0x1720..=0x173F => Hanunoo (v 3);
    84, 0x1740..=0x175F => Buhid (v 3);
    84, 0x1760..=0x177F => Tagbanwa (v 3);
    85, 0x10300..=0x1032F => Old_Italic (v 3);
    86, 0x10330..=0x1034F => Gothic (v 3);
    87, 0x10400..=0x1044F => Deseret (v 3);
    88, 0x1D000..=0x1D0FF => Byzantine_Musical_Symbols (v 3);
    88, 0x1D100..=0x1D1FF => Musical_Symbols (v 3);
    88, 0x1D200..=0x1D24F => Ancient_Greek_Musical_Notation (v 4);
    89, 0x1D400..=0x1D7FF => Mathematical_Alphanumeric_Symbols (v 3);
    90, 0xF0000..=0xFFFFD => Private_Use_Plane_15 (v 3);
    90, 0x100000..=0x10FFFD => Private_Use_Plane_16 (v 3);
    91, 0xFE00..=0xFE0F => Variation_Selectors (v 3);
    91, 0xE0100..=0xE01EF => Variation_Selectors_Supplement (v 3);
    92, 0xE0000..=0xE007F => Tags (v 3);
    93, 0x1900..=0x194F => Limbu (v 4);
    94, 0x1950..=0x197F => Tai_Le (v 4);
    95, 0x1980..=0x19DF => New_Tai_Lue (v 4);
    96, 0x1A00..=0x1A1F => Buginese (v 4);
    97, 0x2C00..=0x2C5F => Glagolitic (v 4);
    98, 0x2D30..=0x2D7F => Tifinagh (v 4);
    99, 0x4DC0..=0x4DFF => Yijing_Hexagram_Symbols (v 4);
    100, 0xA800..=0xA82F => Syloti_Nagri (v 4);
    101, 0x10000..=0x1007F => Linear_B_Syllabary (v 4);
    101, 0x10080..=0x100FF => Linear_B_Ideograms (v 4);
    101, 0x10100..=0x1013F => Aegean_Numbers (v 4);
    102, 0x10140..=0x1018F => Ancient_Greek_Numbers (v 4);
    103, 0x10380..=0x1039F => Ugaritic (v 4);
    104, 0x103A0..=0x103DF => Old_Persian (v 4);
    105, 0x10450..=0x1047F => Shavian (v 4);
    106, 0x10480..=0x104AF => Osmanya (v 4);
    107, 0x10800..=0x1083F => Cypriot_Syllabary (v 4);
    108, 0x10A00..=0x10A5F => Kharoshthi (v 4);
    109, 0x1D300..=0x1D35F => Tai_Xuan_Jing_Symbols (v 4);
    110, 0x12000..=0x123FF => Cuneiform (v 4);
    110, 0x12400..=0x1247F => Cuneiform_Numbers_and_Punctuation (v 4);
    111, 0x1D360..=0x1D37F => Counting_Rod_Numerals (v 4);
    112, 0x1B80..=0x1BBF => Sundanese (v 4);
    113, 0x1C00..=0x1C4F => Lepcha (v 4);
    114, 0x1C50..=0x1C7F => Ol_Chiki (v 4);
    115, 0xA880..=0xA8DF => Saurashtra (v 4);
    116, 0xA900..=0xA92F => Kayah_Li (v 4);
    117, 0xA930..=0xA95F => Rejang (v 4);
    118, 0xAA00..=0xAA5F => Cham (v 4);
    119, 0x10190..=0x101CF => Ancient_Symbols (v 4);
    120, 0x101D0..=0x101FF => Phaistos_Disc (v 4);
    121, 0x102A0..=0x102DF => Carian (v 4);
    121, 0x10280..=0x1029F => Lycian (v 4);
    121, 0x10920..=0x1093F => Lydian (v 4);
    122, 0x1F030..=0x1F09F => Domino_Tiles (v 4);
    122, 0x1F000..=0x1F02F => Mahjong_Tiles (v 4);
    123, 0..=0 => Reserved123;
    124, 0..=0 => Reserved124;
    125, 0..=0 => Reserved125;
    126, 0..=0 => Reserved126;
    127, 0..=0 => Reserved127;
}
