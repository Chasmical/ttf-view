use std::fmt;

/// An [OpenType Tag][spec] consisting of 4 bytes in range `0x20..=0x7E`.
///
/// See constants for commonly used values in the [`tags`] module.
///
/// [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#data-types
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Tag([TagByte; 4]);

// TODO: When pattern types are stabilized and constified, replace TagByte with a pattern type:
// type TagByte = u8 is 0x20 ..= 0x7E;

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum TagByte {
    Space = 0x20, ExclamationMark, QuotationMark, NumberSign, DollarSign, PercentSign, Ampersand,
    Apostrophe, LeftParenthesis, RightParenthesis, Asterisk, PlusSign, Comma, HyphenMinus, FullStop,
    Solidus, Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9, Colon,
    Semicolon, LessThanSign, EqualsSign, GreaterThanSign, QuestionMark, CommercialAt, CapitalA,
    CapitalB, CapitalC, CapitalD, CapitalE, CapitalF, CapitalG, CapitalH, CapitalI, CapitalJ,
    CapitalK, CapitalL, CapitalM, CapitalN, CapitalO, CapitalP, CapitalQ, CapitalR, CapitalS,
    CapitalT, CapitalU, CapitalV, CapitalW, CapitalX, CapitalY, CapitalZ, LeftSquareBracket,
    ReverseSolidus, RightSquareBracket, CircumflexAccent, LowLine, GraveAccent, SmallA, SmallB,
    SmallC, SmallD, SmallE, SmallF, SmallG, SmallH, SmallI, SmallJ, SmallK, SmallL, SmallM, SmallN,
    SmallO, SmallP, SmallQ, SmallR, SmallS, SmallT, SmallU, SmallV, SmallW, SmallX, SmallY, SmallZ,
    LeftCurlyBracket, VerticalLine, RightCurlyBracket, Tilde, // 0x20 ..= 0x7E
}

#[derive(Debug, thiserror::Error)]
#[derive_const(Clone, PartialEq, Eq)]
pub enum ParseTagError {
    #[error("bytes not matching 0x20..=0x7E range")]
    InvalidBytes,
    #[error("tag length is not 3 or 4")]
    InvalidLength,
}

impl Tag {
    /// Converts a 4 byte array to a valid tag. The bytes should be in `0x20..=0x7E` range.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::{ParseTagError, Tag, tags};
    ///
    /// assert_eq!(Tag::from_bytes(*b"cmap"), Ok(tags::cmap));
    /// assert_eq!(Tag::from_bytes(*b"SVG "), Ok(tags::SVG));
    /// assert_eq!(Tag::from_bytes(*b"abc\0"), Err(ParseTagError::InvalidBytes));
    /// ```
    pub const fn from_bytes(bytes: [u8; 4]) -> Result<Self, ParseTagError> {
        if matches!(bytes, [0x20..=0x7E, 0x20..=0x7E, 0x20..=0x7E, 0x20..=0x7E]) {
            Ok(Self(unsafe { std::mem::transmute::<[u8; 4], [TagByte; 4]>(bytes) }))
        } else {
            Err(ParseTagError::InvalidBytes)
        }
    }
    /// Converts an ASCII string to a valid tag. The bytes should be in `0x20..=0x7E` range.
    ///
    /// This function parses 3-char strings as if the fourth byte is a space (`0x20`), allowing,
    /// for example, `"SVG"` to be parsed as `SVG `.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::{ParseTagError, Tag, tags};
    ///
    /// assert_eq!(Tag::from_str("hmtx"), Ok(tags::hmtx));
    /// assert_eq!(Tag::from_str("SVG"), Ok(tags::SVG));
    /// assert_eq!(Tag::from_str("not a tag"), Err(ParseTagError::InvalidLength));
    /// assert_eq!(Tag::from_str(""), Err(ParseTagError::InvalidLength));
    /// ```
    pub const fn from_str(s: &str) -> Result<Self, ParseTagError> {
        Self::from_bytes(match *s.as_bytes() {
            [a, b, c, d] => [a, b, c, d],
            [a, b, c] => [a, b, c, b' '],
            _ => return Err(ParseTagError::InvalidLength),
        })
    }

    /// Returns this tag as a 4 byte array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::tags;
    ///
    /// assert_eq!(tags::BASE.to_bytes(), [b'B', b'A', b'S', b'E']);
    /// assert_eq!(tags::GSUB.to_bytes(), [b'G', b'S', b'U', b'B']);
    /// assert_eq!(tags::CFF.to_bytes(), [b'C', b'F', b'F', b' ']);
    /// ```
    pub const fn to_bytes(self) -> [u8; 4] {
        unsafe { std::mem::transmute(self.0) }
    }
    /// Returns a reference to this tag's 4 byte array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::tags;
    ///
    /// assert_eq!(tags::cvt.as_bytes(), b"cvt ");
    /// assert_eq!(tags::COLR.as_bytes(), b"COLR");
    /// assert_eq!(tags::OS_2.as_bytes(), b"OS/2");
    /// ```
    pub const fn as_bytes(&self) -> &[u8; 4] {
        unsafe { std::mem::transmute(&self.0) }
    }
    /// Returns a reference to this tag's 4 byte array as a UTF-8 encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::tags;
    ///
    /// assert_eq!(tags::cvt.as_str(), "cvt ");
    /// assert_eq!(tags::COLR.as_str(), "COLR");
    /// assert_eq!(tags::OS_2.as_str(), "OS/2");
    /// ```
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}

macro_rules! define_known_tags {
    ( $($tag:ident $(= $s:expr)?),* $(,)? ) => {
        impl Tag {
            /// An array of all known OpenType tags, collected from Microsoft's OpenType spec.
            ///
            /// Its contents and order may and probably will change in the future.
            pub const KNOWN_TAGS: &[Tag] = &[ $( tags::$tag, )* ];

            /// Checks if this tag is a known OpenType tag ([`Tag::KNOWN_TAGS`]).
            ///
            /// ```
            /// use ttf_view::types::{Tag, tags};
            ///
            /// // All tags in the `tags` module are known
            /// assert_eq!(tags::name.is_known(), true);
            /// assert_eq!(Tag::from_str("name").unwrap().is_known(), true);
            /// assert_eq!(Tag::from_str("XXXX").unwrap().is_known(), false);
            /// ```
            pub const fn is_known(&self) -> bool {
                matches!(*self, $( tags::$tag )|* )
            }
        }

        #[allow(non_upper_case_globals)]
        pub mod tags {
            use super::Tag;

            $( pub const $tag: Tag = Tag::from_str(define_known_tags!(@value $tag $(= $s)?)).ok().unwrap(); )*
        }
    };
    (@value $tag:ident) => (stringify!($tag));
    (@value $tag:ident = $s:expr) => ($s);
}

define_known_tags! {
    avar, BASE, CBDT, CBLC, CFF, CFF2, cmap, COLR, CPAL, cvar, cvt, DSIG, EBDT, EBLC, EBSC, fpgm,
    fvar, gasp, GDEF, glyf, GPOS, GSUB, gvar, hdmx, head, hhea, hmtx, HVAR, JSTF, kern, loca, LTSH,
    MATH, maxp, MERG, meta, MVAR, name, OS_2 = "OS/2", PCLT, post, prep, sbix, STAT, SVG, VDMX,
    vhea, vmtx, VORG, VVAR,
}

/// Formats the tag's value surrounded by apostrophes: e.g. `'COLR'`, `'cvt '`, `'glyf'`.
impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = [b'\'', 0, 0, 0, 0, b'\''];
        buf[1..5].copy_from_slice(self.as_bytes());
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}
/// Formats the tag's value as an unchanged string: e.g. `COLR`, `cvt `, `glyf`.
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

const impl PartialEq<str> for Tag {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

const impl std::str::FromStr for Tag {
    type Err = ParseTagError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
const impl TryFrom<&str> for Tag {
    type Error = ParseTagError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
const impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
