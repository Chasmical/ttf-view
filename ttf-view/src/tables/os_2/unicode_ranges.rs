macro_rules! define_unicode_ranges {
    ($(
        $from:literal ..= $to:literal, $bit_index:literal => $field:ident (v $version:literal);
    )*) => {
        bitflags::bitflags! {
            // TODO: When bitflags::Bits's Clone + PartialEq + Eq are constified, make derives const
            #[derive(Clone, Copy, PartialEq, Eq, Hash)]
            pub struct UnicodeRanges: u128 {
                $( const $field = 1 << $bit_index; )*
            }
        }

        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq)]
        #[repr(u8)]
        #[non_exhaustive]
        pub enum UnicodeRange {
            $(#[doc(hidden)] $field,)*
        }

        impl UnicodeRange {
            #[allow(unreachable_patterns)]
            pub const fn from_char(ch: char) -> Option<Self> {
                Some(match ch as u32 { $($from..=$to => Self::$field,)* _ => return None })
            }
            pub const fn bit_index(&self) -> u8 {
                match self { $(Self::$field => $bit_index,)* }
            }
            pub const fn name(&self) -> &'static str {
                match self { $(Self::$field => stringify!($field),)* }
            }
            pub const fn range(&self) -> std::ops::RangeInclusive<u32> {
                match self { $(Self::$field => $from..=$to,)* }
            }
            pub const fn version(&self) -> u8 {
                match self { $(Self::$field => $version,)* }
            }
        }
    };
}

impl UnicodeRanges {
    pub const fn from_parts(ul1: u32, ul2: u32, ul3: u32, ul4: u32) -> Self {
        Self::from_bits_retain(
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

    pub fn from_chars(iter: impl IntoIterator<Item = char>, version: u8) -> Self {
        iter.into_iter().fold(Self::empty(), |mut acc, ch| {
            if let Some(range) = UnicodeRange::from_char(ch)
                && version >= range.version()
            {
                acc |= range;
            }
            if matches!(ch as u32, 0x10000..=0x10FFFF) {
                acc |= UnicodeRange::Non_Plane_0;
            }
            acc
        })
    }

    // TODO: iter UnicodeRanges
}

// TODO: impl Debug for UnicodeRange and UnicodeRanges

const impl From<UnicodeRange> for UnicodeRanges {
    fn from(value: UnicodeRange) -> Self {
        Self::from_bits_retain(1 << value.bit_index())
    }
}
const impl std::ops::BitOr<UnicodeRange> for UnicodeRanges {
    type Output = Self;
    fn bitor(self, rhs: UnicodeRange) -> Self::Output {
        self.union(Self::from(rhs))
    }
}
const impl std::ops::BitOrAssign<UnicodeRange> for UnicodeRanges {
    fn bitor_assign(&mut self, rhs: UnicodeRange) {
        *self = self.union(Self::from(rhs));
    }
}

define_unicode_ranges! {
    // OS/2 v2 = OT v1.3
    // OS/2 v3 = OT v1.4
    // OS/2 v4 = OT v1.5
    //
    // bits 123-127 are reserved

    0x0000..=0x007F, 0 => Basic_Latin (v 1);
    0x0080..=0x00FF, 1 => Latin_1_Supplement (v 1);
    0x0100..=0x017F, 2 => Latin_Extended_A (v 1);
    0x0180..=0x024F, 3 => Latin_Extended_B (v 1);
    0x0250..=0x02AF, 4 => IPA_Extensions (v 1);
    0x02B0..=0x02FF, 5 => Spacing_Modifier_Letters (v 1);
    0x0300..=0x036F, 6 => Combining_Diacritical_Marks (v 1);
    0x0370..=0x03FF, 7 => Greek_and_Coptic (v 1); // v1: non-well-defined as "Basic Greek"
    0x0400..=0x04FF, 9 => Cyrillic (v 1);
    0x0500..=0x052F, 9 => Cyrillic_Supplement (v 3);
    0x0530..=0x058F, 10 => Armenian (v 1);
    0x0590..=0x05FF, 11 => Hebrew (v 1); // v1: non-well-defined as "Basic Hebrew"
    0x0600..=0x06FF, 13 => Arabic (v 1); // v1: non-well-defined as "Basic Arabic"
    0x0700..=0x074F, 71 => Syriac (v 2);
    0x0750..=0x077F, 13 => Arabic_Supplement (v 4);
    0x0780..=0x07BF, 72 => Thaana (v 2);
    0x07C0..=0x07FF, 14 => NKo (v 4); // v1: non-well-defined as "Arabic Extended"
    // 0x0800..=0x08FF not mapped
    0x0900..=0x097F, 15 => Devanagari (v 1);
    0x0980..=0x09FF, 16 => Bangla (v 1);
    0x0A00..=0x0A7F, 17 => Gurmukhi (v 1);
    0x0A80..=0x0AFF, 18 => Gujarati (v 1);
    0x0B00..=0x0B7F, 19 => Odia (v 1);
    0x0B80..=0x0BFF, 20 => Tamil (v 1);
    0x0C00..=0x0C7F, 21 => Telugu (v 1);
    0x0C80..=0x0CFF, 22 => Kannada (v 1);
    0x0D00..=0x0D7F, 23 => Malayalam (v 1);
    0x0D80..=0x0DFF, 73 => Sinhala (v 2);
    0x0E00..=0x0E7F, 24 => Thai (v 1);
    0x0E80..=0x0EFF, 25 => Lao (v 1);
    0x0F00..=0x0FFF, 70 => Tibetan (v 2);
    0x1000..=0x109F, 74 => Myanmar (v 2);
    0x10A0..=0x10FF, 26 => Georgian (v 1); // v1: non-well-defined as "Basic Georgian"
    0x1100..=0x11FF, 28 => Hangul_Jamo (v 1);
    0x1200..=0x137F, 75 => Ethiopic (v 2);
    0x1380..=0x139F, 75 => Ethiopic_Supplement (v 4);
    0x13A0..=0x13FF, 76 => Cherokee (v 2);
    0x1400..=0x167F, 77 => Unified_Canadian_Aboriginal_Syllabics (v 2);
    0x1680..=0x169F, 78 => Ogham (v 2);
    0x16A0..=0x16FF, 79 => Runic (v 2);
    0x1700..=0x171F, 84 => Tagalog (v 3);
    0x1720..=0x173F, 84 => Hanunoo (v 3);
    0x1740..=0x175F, 84 => Buhid (v 3);
    0x1760..=0x177F, 84 => Tagbanwa (v 3);
    0x1780..=0x17FF, 80 => Khmer (v 2);
    0x1800..=0x18AF, 81 => Mongolian (v 2);
    // 0x18B0..=0x18FF not mapped
    0x1900..=0x194F, 93 => Limbu (v 4);
    0x1950..=0x197F, 94 => Tai_Le (v 4);
    0x1980..=0x19DF, 95 => New_Tai_Lue (v 4);
    0x19E0..=0x19FF, 80 => Khmer_Symbols (v 4);
    0x1A00..=0x1A1F, 96 => Buginese (v 4);
    // 0x1A20..=0x1AFF not mapped
    0x1B00..=0x1B7F, 27 => Balinese (v 4); // v1: non-well-defined as "Georgian Extended"
    0x1B80..=0x1BBF, 112 => Sundanese (v 4);
    // 0x1BC0..=0x1BFF not mapped
    0x1C00..=0x1C4F, 113 => Lepcha (v 4);
    0x1C50..=0x1C7F, 114 => Ol_Chiki (v 4);
    // 0x1C80..=0x1CFF not mapped
    0x1D00..=0x1D7F, 4 => Phonetic_Extensions (v 4);
    0x1D80..=0x1DBF, 4 => Phonetic_Extensions_Supplement (v 4);
    0x1DC0..=0x1DFF, 6 => Combining_Diacritical_Marks_Supplement (v 4);
    0x1E00..=0x1EFF, 29 => Latin_Extended_Additional (v 1);
    0x1F00..=0x1FFF, 30 => Greek_Extended (v 1);
    0x2000..=0x206F, 31 => General_Punctuation (v 1);
    0x2070..=0x209F, 32 => Superscripts_And_Subscripts (v 1);
    0x20A0..=0x20CF, 33 => Currency_Symbols (v 1);
    0x20D0..=0x20FF, 34 => Combining_Diacritical_Marks_For_Symbols (v 1);
    0x2100..=0x214F, 35 => Letterlike_Symbols (v 1);
    0x2150..=0x218F, 36 => Number_Forms (v 1);
    0x2190..=0x21FF, 37 => Arrows (v 1);
    0x2200..=0x22FF, 38 => Mathematical_Operators (v 1);
    0x2300..=0x23FF, 39 => Miscellaneous_Technical (v 1);
    0x2400..=0x243F, 40 => Control_Pictures (v 1);
    0x2440..=0x245F, 41 => Optical_Character_Recognition (v 1);
    0x2460..=0x24FF, 42 => Enclosed_Alphanumerics (v 1);
    0x2500..=0x257F, 43 => Box_Drawing (v 1);
    0x2580..=0x259F, 44 => Block_Elements (v 1);
    0x25A0..=0x25FF, 45 => Geometric_Shapes (v 1);
    0x2600..=0x26FF, 46 => Miscellaneous_Symbols (v 1);
    0x2700..=0x27BF, 47 => Dingbats (v 1);
    0x27C0..=0x27EF, 38 => Miscellaneous_Mathematical_Symbols_A (v 3);
    0x27F0..=0x27FF, 37 => Supplemental_Arrows_A (v 3);
    0x2800..=0x28FF, 82 => Braille_Patterns (v 2);
    0x2900..=0x297F, 37 => Supplemental_Arrows_B (v 3);
    0x2980..=0x29FF, 38 => Miscellaneous_Mathematical_Symbols_B (v 3);
    0x2A00..=0x2AFF, 38 => Supplemental_Mathematical_Operators (v 3);
    0x2B00..=0x2BFF, 37 => Miscellaneous_Symbols_and_Arrows (v 4);
    0x2C00..=0x2C5F, 97 => Glagolitic (v 4);
    0x2C60..=0x2C7F, 29 => Latin_Extended_C (v 4);
    0x2C80..=0x2CFF, 8 => Coptic (v 4); // v1: non-well-defined as "Greek Symbols and Coptic"
    0x2D00..=0x2D2F, 26 => Georgian_Supplement (v 4);
    0x2D30..=0x2D7F, 98 => Tifinagh (v 4);
    0x2D80..=0x2DDF, 75 => Ethiopic_Extended (v 4);
    0x2DE0..=0x2DFF, 9 => Cyrillic_Extended_A (v 4);
    0x2E00..=0x2E7F, 31 => Supplemental_Punctuation (v 4);
    0x2E80..=0x2EFF, 59 => CJK_Radicals_Supplement (v 2);
    0x2F00..=0x2FDF, 59 => Kangxi_Radicals (v 2);
    // 0x2FE0..=0x2FEF not mapped
    0x2FF0..=0x2FFF, 59 => Ideographic_Description_Characters (v 2);
    0x3000..=0x303F, 48 => CJK_Symbols_And_Punctuation (v 1);
    0x3040..=0x309F, 49 => Hiragana (v 1);
    0x30A0..=0x30FF, 50 => Katakana (v 1);
    0x3100..=0x312F, 51 => Bopomofo (v 1);
    0x3130..=0x318F, 52 => Hangul_Compatibility_Jamo (v 1);
    0x3190..=0x319F, 59 => Kanbun (v 3);
    0x31A0..=0x31BF, 51 => Bopomofo_Extended (v 2);
    0x31C0..=0x31EF, 61 => CJK_Strokes (v 4);
    0x31F0..=0x31FF, 50 => Katakana_Phonetic_Extensions (v 3);
    0x3200..=0x32FF, 54 => Enclosed_CJK_Letters_And_Months (v 1);
    0x3300..=0x33FF, 55 => CJK_Compatibility (v 1);
    0x3400..=0x4DBF, 59 => CJK_Unified_Ideographs_Extension_A (v 2);
    0x4DC0..=0x4DFF, 99 => Yijing_Hexagram_Symbols (v 4);
    0x4E00..=0x9FFF, 59 => CJK_Unified_Ideographs (v 1);
    0xA000..=0xA48F, 83 => Yi_Syllables (v 2);
    0xA490..=0xA4CF, 83 => Yi_Radicals (v 2);
    // 0xA4D0..=0xA4FF not mapped
    0xA500..=0xA63F, 12 => Vai (v 4); // v1: non-well-defined as "Hebrew Extended"
    0xA640..=0xA69F, 9 => Cyrillic_Extended_B (v 4);
    // 0xA6A0..=0xA6FF not mapped
    0xA700..=0xA71F, 5 => Modifier_Tone_Letters (v 4);
    0xA720..=0xA7FF, 29 => Latin_Extended_D (v 4);
    0xA800..=0xA82F, 100 => Syloti_Nagri (v 4);
    // 0xA830..=0xA83F not mapped
    0xA840..=0xA87F, 53 => Phags_pa (v 4); // v1/v2: non-well-defined as "CJK Miscellaneous"
    0xA880..=0xA8DF, 115 => Saurashtra (v 4);
    // 0xA8E0..=0xA8FF not mapped
    0xA900..=0xA92F, 116 => Kayah_Li (v 4);
    0xA930..=0xA95F, 117 => Rejang (v 4);
    // 0xA960..=0xA9FF not mapped
    0xAA00..=0xAA5F, 118 => Cham (v 4);
    // 0xAA60..=0xABFF not mapped
    0xAC00..=0xD7AF, 56 => Hangul_Syllables (v 1);
    // 0xD7B0..=0xD7FF not mapped
    // 0xD800..=0xDFFF are UTF-16 surrogates
    0xE000..=0xF8FF, 60 => Private_Use_Area_Plane_0 (v 1);
    0xF900..=0xFAFF, 61 => CJK_Compatibility_Ideographs (v 1);
    0xFB00..=0xFB4F, 62 => Alphabetic_Presentation_Forms (v 1);
    0xFB50..=0xFDFF, 63 => Arabic_Presentation_Forms_A (v 1);
    0xFE00..=0xFE0F, 91 => Variation_Selectors (v 3);
    0xFE10..=0xFE1F, 65 => Vertical_Forms (v 4);
    0xFE20..=0xFE2F, 64 => Combining_Half_Marks (v 1);
    0xFE30..=0xFE4F, 65 => CJK_Compatibility_Forms (v 1);
    0xFE50..=0xFE6F, 66 => Small_Form_Variants (v 1);
    0xFE70..=0xFEFF, 67 => Arabic_Presentation_Forms_B (v 1);
    0xFF00..=0xFFEF, 68 => Halfwidth_And_Fullwidth_Forms (v 1);
    0xFFF0..=0xFFFF, 69 => Specials (v 1);
    0x10000..=0x1007F, 101 => Linear_B_Syllabary (v 4);
    0x10080..=0x100FF, 101 => Linear_B_Ideograms (v 4);
    0x10100..=0x1013F, 101 => Aegean_Numbers (v 4);
    0x10140..=0x1018F, 102 => Ancient_Greek_Numbers (v 4);
    0x10190..=0x101CF, 119 => Ancient_Symbols (v 4);
    0x101D0..=0x101FF, 120 => Phaistos_Disc (v 4);
    // 0x10200..=0x1027F not mapped
    0x10280..=0x1029F, 121 => Lycian (v 4);
    0x102A0..=0x102DF, 121 => Carian (v 4);
    // 0x102E0..=0x102FF not mapped
    0x10300..=0x1032F, 85 => Old_Italic (v 3);
    0x10330..=0x1034F, 86 => Gothic (v 3);
    // 0x10350..=0x1037F not mapped
    0x10380..=0x1039F, 103 => Ugaritic (v 4);
    0x103A0..=0x103DF, 104 => Old_Persian (v 4);
    // 0x103E0..=0x103FF not mapped
    0x10400..=0x1044F, 87 => Deseret (v 3);
    0x10450..=0x1047F, 105 => Shavian (v 4);
    0x10480..=0x104AF, 106 => Osmanya (v 4);
    // 0x104B0..=0x107FF not mapped
    0x10800..=0x1083F, 107 => Cypriot_Syllabary (v 4);
    // 0x10840..=0x108FF not mapped
    0x10900..=0x1091F, 58 => Phoenician (v 4);
    0x10920..=0x1093F, 121 => Lydian (v 4);
    // 0x10940..=0x109FF not mapped
    0x10A00..=0x10A5F, 108 => Kharoshthi (v 4);
    // 0x10A60..=0x11FFF not mapped
    0x12000..=0x123FF, 110 => Cuneiform (v 4);
    0x12400..=0x1247F, 110 => Cuneiform_Numbers_and_Punctuation (v 4);
    // 0x12480..=0x1CFFF not mapped
    0x1D000..=0x1D0FF, 88 => Byzantine_Musical_Symbols (v 3);
    0x1D100..=0x1D1FF, 88 => Musical_Symbols (v 3);
    0x1D200..=0x1D24F, 88 => Ancient_Greek_Musical_Notation (v 4);
    // 0x1D250..=0x1D2FF not mapped
    0x1D300..=0x1D35F, 109 => Tai_Xuan_Jing_Symbols (v 4);
    0x1D360..=0x1D37F, 111 => Counting_Rod_Numerals (v 4);
    // 0x1D380..=0x1D3FF not mapped
    0x1D400..=0x1D7FF, 89 => Mathematical_Alphanumeric_Symbols (v 3);
    // 0x1D800..=0x1EFFF not mapped
    0x1F000..=0x1F02F, 122 => Mahjong_Tiles (v 4);
    0x1F030..=0x1F09F, 122 => Domino_Tiles (v 4);
    // 0x1F0A0..=0x1FFFF not mapped
    0x20000..=0x2A6DF, 59 => CJK_Unified_Ideographs_Extension_B (v 3);
    // 0x2A6E0..=0x2F7FF not mapped
    0x2F800..=0x2FA1F, 61 => CJK_Compatibility_Ideographs_Supplement (v 3);
    // 0x2FA20..=0xDFFFF not mapped
    0xE0000..=0xE007F, 92 => Tags (v 3);
    // 0xE0080..=0xE00FF not mapped
    0xE0100..=0xE01EF, 91 => Variation_Selectors_Supplement (v 3);
    // 0xE01F0..=0xEFFFF not mapped
    0xF0000..=0xFFFFD, 90 => Private_Use_Plane_15 (v 3);
    // 0xFFFFE..=0xFFFFF not mapped
    0x100000..=0x10FFFD, 90 => Private_Use_Plane_16 (v 3);

    // Note: Non_Plane_0 overlaps other ranges (it's the only one to do so), so it must be located
    // at the end, so that it will only catch chars that weren't in any of the more specific ranges.
    0x10000..=0x10FFFF, 57 => Non_Plane_0 (v 2);
}
