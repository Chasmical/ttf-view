#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct CodePages(u64);

impl CodePages {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }
    pub const fn bits(&self) -> u64 {
        self.0
    }

    pub const fn from_parts(ul1: u32, ul2: u32) -> Self {
        Self((ul1 as u64) | ((ul2 as u64) << 32))
    }
    pub const fn into_parts(self) -> (u32, u32) {
        (self.bits() as u32, (self.bits() >> 32) as u32)
    }

    // TODO: iter CodePages
}

const impl From<CodePage> for CodePages {
    fn from(value: CodePage) -> Self {
        Self(1 << value.bit_index())
    }
}
const impl std::ops::BitOr for CodePages {
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
    type Output = Self;
}
const impl std::ops::BitOrAssign for CodePages {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0);
    }
}
const impl std::ops::BitOr<CodePage> for CodePages {
    fn bitor(self, rhs: CodePage) -> Self::Output {
        self.bitor(Self::from(rhs))
    }
    type Output = Self;
}
const impl std::ops::BitOrAssign<CodePage> for CodePages {
    fn bitor_assign(&mut self, rhs: CodePage) {
        self.bitor_assign(Self::from(rhs));
    }
}

macro_rules! define_codepages {
    ($(
        $bit_index:literal, $codepage:literal => $field:ident;
    )*) => {
        #[repr(u8)]
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum CodePage {
            $(#[doc(hidden)] $field,)*
        }

        impl CodePage {
            pub const fn bit_index(&self) -> u8 {
                match self {
                    $(Self::$field => $bit_index,)*
                }
            }
            pub const fn codepage(&self) -> Option<u16> {
                let val = match self {
                    $(Self::$field => $codepage,)*
                };
                if val != 0 { Some(val) } else { None }
            }
        }
    };
}

define_codepages! {
    0, 1252 => Latin_1;
    1, 1250 => Latin_2_Eastern_Europe;
    2, 1251 => Cyrillic;
    3, 1253 => Greek;
    4, 1254 => Turkish;
    5, 1255 => Hebrew;
    6, 1256 => Arabic;
    7, 1257 => Windows_Baltic;
    8, 1258 => Vietnamese;
    // 9-15 reserved for Alternate ANSI
    9, 0 => Reserved9;
    10, 0 => Reserved10;
    11, 0 => Reserved11;
    12, 0 => Reserved12;
    13, 0 => Reserved13;
    14, 0 => Reserved14;
    15, 0 => Reserved15;
    16, 874 => Thai;
    17, 932 => JIS_Japan;
    18, 936 => Chinese_Simplified_chars_PRC_and_Singapore;
    19, 949 => Korean_Wansung;
    20, 950 => Chinese_Traditional_chars_Taiwan_and_Hong_Kong_SAR;
    21, 1361 => Korean_Johab;
    // 22-28 reserved for Alternate ANSI or OEM
    22, 0 => Reserved22;
    23, 0 => Reserved23;
    24, 0 => Reserved24;
    25, 0 => Reserved25;
    26, 0 => Reserved26;
    27, 0 => Reserved27;
    28, 0 => Reserved28;
    29, 0 => Macintosh_Character_Set_US_Roman;
    30, 0 => OEM_Character_Set;
    31, 0 => Symbol_Character_Set;
    // 32-47 reserved for OEM
    32, 0 => Reserved32;
    33, 0 => Reserved33;
    34, 0 => Reserved34;
    35, 0 => Reserved35;
    36, 0 => Reserved36;
    37, 0 => Reserved37;
    38, 0 => Reserved38;
    39, 0 => Reserved39;
    40, 0 => Reserved40;
    41, 0 => Reserved41;
    42, 0 => Reserved42;
    43, 0 => Reserved43;
    44, 0 => Reserved44;
    45, 0 => Reserved45;
    46, 0 => Reserved46;
    47, 0 => Reserved47;
    48, 869 => IBM_Greek;
    49, 866 => MS_DOS_Russian;
    50, 865 => MS_DOS_Nordic;
    51, 864 => Arabic2;
    52, 863 => MS_DOS_Canadian_French;
    53, 862 => Hebrew2;
    54, 861 => MS_DOS_Icelandic;
    55, 860 => MS_DOS_Portuguese;
    56, 857 => IBM_Turkish;
    57, 855 => IBM_Cyrillic_primarily_Russian;
    58, 852 => Latin_2;
    59, 775 => MS_DOS_Baltic;
    60, 737 => Greek_former_437_G;
    61, 708 => Arabic_ASMO_708;
    62, 850 => WE_Latin_1;
    63, 437 => US;
}
