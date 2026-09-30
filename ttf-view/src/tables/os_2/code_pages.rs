macro_rules! define_codepages {
    ($(
        $bit_index:literal, $code_page:literal => $field:ident;
    )*) => {
        bitflags::bitflags! {
            // TODO: When bitflags::Bits's Clone + PartialEq + Eq are constified, make derives const
            #[derive(Clone, Copy, PartialEq, Eq, Hash)]
            pub struct CodePages: u64 {
                $( const $field = 1 << $bit_index; )*
            }
        }

        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq)]
        #[repr(u8)]
        #[non_exhaustive]
        pub enum CodePage {
            $(#[doc(hidden)] $field = $bit_index,)*
        }

        impl CodePage {
            pub const fn from_bit_index(bit_index: u8) -> Option<Self> {
                Some(match bit_index { $($bit_index => Self::$field,)+ _ => return None })
            }
            #[allow(unreachable_patterns)]
            pub const fn from_code_page(code_page: u16) -> Option<Self> {
                Some(match code_page { 0 => return None, $($code_page => Self::$field,)+ _ => return None })
            }
            pub const fn bit_index(self) -> u8 {
                self as u8
            }
            pub const fn name(&self) -> &'static str {
                match self { $(Self::$field => stringify!($field),)* }
            }
            pub const fn code_page(&self) -> Option<u16> {
                let code_page = match self { $(Self::$field => $code_page,)* };
                if code_page != 0 { Some(code_page) } else { None }
            }
        }
    };
}

impl CodePages {
    pub const fn from_parts(ul1: u32, ul2: u32) -> Self {
        Self::from_bits_retain((ul1 as u64) | ((ul2 as u64) << 32))
    }
    pub const fn into_parts(self) -> (u32, u32) {
        (self.bits() as u32, (self.bits() >> 32) as u32)
    }

    // TODO: iter CodePages
}

// TODO: impl Debug for CodePage and CodePages

const impl From<CodePage> for CodePages {
    fn from(value: CodePage) -> Self {
        Self::from_bits_retain(1 << value.bit_index())
    }
}
const impl std::ops::BitOr<CodePage> for CodePages {
    type Output = Self;
    fn bitor(self, rhs: CodePage) -> Self::Output {
        self.union(Self::from(rhs))
    }
}
const impl std::ops::BitOrAssign<CodePage> for CodePages {
    fn bitor_assign(&mut self, rhs: CodePage) {
        *self = self.union(Self::from(rhs));
    }
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
    16, 874 => Thai;
    17, 932 => JIS_Japan;
    18, 936 => Chinese_Simplified_chars_PRC_and_Singapore;
    19, 949 => Korean_Wansung;
    20, 950 => Chinese_Traditional_chars_Taiwan_and_Hong_Kong_SAR;
    21, 1361 => Korean_Johab;
    // 22-28 reserved for Alternate ANSI or OEM
    29, 0 => Macintosh_Character_Set_US_Roman;
    30, 0 => OEM_Character_Set;
    31, 0 => Symbol_Character_Set;
    // 32-47 reserved for OEM
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
