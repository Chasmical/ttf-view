use std::{fmt, num::ParseIntError};

/// An [OpenType Version16Dot16][spec] version.
///
/// The upper 16 bits comprise a major version number, and the highest-order nibble (4 bits) of the
/// lower 16 bits comprises a minor version number (0..=9 range). Simply put, the major version is
/// in `0xFFFF0000`, and the minor version is in `0x0000F000`.
///
/// [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-version-numbers
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Version16Dot16([u8; 4]);

impl Version16Dot16 {
    /// Version 0.5.
    pub const V0_5: Self = Self::new(0, 5).unwrap();
    /// Version 1.0.
    pub const V1_0: Self = Self::new(1, 0).unwrap();

    /// Creates a [`Version16Dot16`] from major and minor version numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(Version16Dot16::new(0, 5).unwrap().tuple(), (0, 5));
    /// assert_eq!(Version16Dot16::new(15, 9).unwrap().tuple(), (15, 9));
    /// assert_eq!(Version16Dot16::new(0xFFFF, 9).unwrap().tuple(), (0xFFFF, 9));
    /// assert_eq!(Version16Dot16::new(15, 10), None);
    /// assert_eq!(Version16Dot16::new(1, 199), None);
    /// ```
    pub const fn new(major: u16, minor: u8) -> Option<Self> {
        if minor <= 9 { Some(unsafe { Self::new_unchecked(major, minor) }) } else { None }
    }
    /// Creates a [`Version16Dot16`] from major and minor version numbers without checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(unsafe { Version16Dot16::new_unchecked(0, 5).tuple() }, (0, 5));
    /// assert_eq!(unsafe { Version16Dot16::new_unchecked(0xFFFF, 9).tuple() }, (0xFFFF, 9));
    /// ```
    pub const unsafe fn new_unchecked(major: u16, minor: u8) -> Self {
        debug_assert!(minor <= 9);
        let raw = ((major as u32) << 16) | ((minor as u32) << 12);
        Self(raw.to_be_bytes())
    }

    /// Creates a [`Version16Dot16`] from big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(Version16Dot16::from_be_bytes([0x00, 0x00, 0x50, 0x00]).unwrap().tuple(), (0, 5));
    /// assert_eq!(Version16Dot16::from_be_bytes([0x00, 0x01, 0x90, 0x00]).unwrap().tuple(), (1, 9));
    /// assert_eq!(Version16Dot16::from_be_bytes([0x12, 0x34, 0x90, 0x00]).unwrap().tuple(), (0x1234, 9));
    /// assert_eq!(Version16Dot16::from_be_bytes([0x00, 0x01, 0xA0, 0x00]), None);
    /// assert_eq!(Version16Dot16::from_be_bytes([0x00, 0x01, 0x90, 0x64]), None);
    /// ```
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Option<Self> {
        match bytes {
            [_, _, x @ 0x00..=0x90, 0] if (x & 0x0F) == 0 => Some(Self(bytes)),
            _ => None,
        }
    }
    /// Creates a [`Version16Dot16`] from big-endian bytes without checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(unsafe { Version16Dot16::from_be_bytes_unchecked([0x00, 0x00, 0x50, 0x00]).tuple() }, (0, 5));
    /// assert_eq!(unsafe { Version16Dot16::from_be_bytes_unchecked([0x00, 0x01, 0x90, 0x00]).tuple() }, (1, 9));
    /// assert_eq!(unsafe { Version16Dot16::from_be_bytes_unchecked([0x12, 0x34, 0x90, 0x00]).tuple() }, (0x1234, 9));
    /// ```
    pub const unsafe fn from_be_bytes_unchecked(bytes: [u8; 4]) -> Self {
        debug_assert!(Self::from_be_bytes(bytes).is_some());
        Self(bytes)
    }
    /// Gets this [`Version16Dot16`]'s big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(Version16Dot16::new(1, 5).unwrap().to_be_bytes(), [0x00, 0x01, 0x50, 0x00]);
    /// assert_eq!(Version16Dot16::new(0x1234, 5).unwrap().to_be_bytes(), [0x12, 0x34, 0x50, 0x00]);
    /// assert_eq!(Version16Dot16::new(0x1234, 9).unwrap().to_be_bytes(), [0x12, 0x34, 0x90, 0x00]);
    /// ```
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0
    }

    /// Gets this [`Version16Dot16`]'s major version number.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(Version16Dot16::new(0, 9).unwrap().major(), 0);
    /// assert_eq!(Version16Dot16::new(13, 2).unwrap().major(), 13);
    /// assert_eq!(Version16Dot16::new(0xFFFF, 5).unwrap().major(), 0xFFFF);
    /// ```
    pub const fn major(&self) -> u16 {
        u16::from_be_bytes(*self.0.first_chunk::<2>().unwrap())
    }
    /// Gets this [`Version16Dot16`]'s minor version number.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(Version16Dot16::new(0, 9).unwrap().minor(), 9);
    /// assert_eq!(Version16Dot16::new(13, 2).unwrap().minor(), 2);
    /// assert_eq!(Version16Dot16::new(0xFFFF, 5).unwrap().minor(), 5);
    /// ```
    pub const fn minor(&self) -> u8 {
        self.0[2] >> 4
    }
    /// Gets this [`Version16Dot16`]'s major and minor version numbers as a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Version16Dot16;
    ///
    /// assert_eq!(Version16Dot16::new(0, 9).unwrap().tuple(), (0, 9));
    /// assert_eq!(Version16Dot16::new(13, 2).unwrap().tuple(), (13, 2));
    /// assert_eq!(Version16Dot16::new(0xFFFF, 5).unwrap().tuple(), (0xFFFF, 5));
    /// ```
    pub const fn tuple(&self) -> (u16, u8) {
        (self.major(), self.minor())
    }
}

// TODO: When [u8; 4]'s Default is constified, replace this impl with #[derive_const]
#[allow(clippy::derivable_impls)]
const impl Default for Version16Dot16 {
    fn default() -> Self {
        Self([0; 4])
    }
}

impl fmt::Debug for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl fmt::Display for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major(), self.minor())
    }
}
impl fmt::LowerHex for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        u32::from_be_bytes(self.to_be_bytes()).fmt(f)
    }
}
impl fmt::UpperHex for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        u32::from_be_bytes(self.to_be_bytes()).fmt(f)
    }
}

/// Error type for parsing [`Version16Dot16`].
// TODO: When ParseIntError's Clone + PartialEq + Eq are constified, make derives const
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseVersion16Dot16Error {
    /// The string doesn't contain a dot `.`.
    #[error("dot not found in string")]
    NoDot,
    /// The minor version is not in 0..=9 range.
    #[error("minor version is not in 0..=9 range")]
    Invalid,
    /// Could not parse major/minor version component.
    #[error("number parsing error: {0}")]
    Number(ParseIntError),
}

impl std::str::FromStr for Version16Dot16 {
    type Err = ParseVersion16Dot16Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s.split_once('.').ok_or(ParseVersion16Dot16Error::NoDot)?;
        Self::new(
            major.parse().map_err(ParseVersion16Dot16Error::Number)?,
            minor.parse().map_err(ParseVersion16Dot16Error::Number)?,
        )
        .ok_or(ParseVersion16Dot16Error::Invalid)
    }
}
