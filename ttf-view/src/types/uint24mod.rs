use crate::util::impl_fmt_with;

/// The 24-bit unsigned integer type stored in big-endian byte order.
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct uint24([u8; 3]);

impl uint24 {
    /// The size of this integer type in bits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ttf_view::types::uint24;
    /// assert_eq!(uint24::BITS, 24);
    /// ```
    pub const BITS: u32 = 24;
    /// The smallest value that can be represented by this integer type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ttf_view::types::uint24;
    /// assert_eq!(uint24::MIN.get(), 0);
    /// ```
    pub const MIN: Self = Self::new(0x000000).unwrap();
    /// The largest value that can be represented by this integer type (2<sup>24</sup> &minus; 1).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ttf_view::types::uint24;
    /// assert_eq!(uint24::MAX.get(), 16_777_215);
    /// ```
    pub const MAX: Self = Self::new(0xFFFFFF).unwrap();

    /// Creates a big-endian [`uint24`] from [`u32`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::uint24;
    ///
    /// assert_eq!(uint24::new(1234).unwrap().get(), 1234);
    /// assert_eq!(uint24::new(15_000_000).unwrap().get(), 15_000_000);
    /// assert_eq!(uint24::new(0xFFFFFF).unwrap().get(), 0xFFFFFF);
    /// assert_eq!(uint24::new(17_000_000), None);
    /// assert_eq!(uint24::new(3_999_000_000), None);
    /// ```
    pub const fn new(num: u32) -> Option<Self> {
        if num <= 0xFFFFFF { Some(unsafe { Self::new_unchecked(num) }) } else { None }
    }
    /// Creates a big-endian [`uint24`] from [`u32`] without checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::uint24;
    ///
    /// assert_eq!(unsafe { uint24::new_unchecked(1234).get() }, 1234);
    /// assert_eq!(unsafe { uint24::new_unchecked(15_000_000).get() }, 15_000_000);
    /// ```
    pub const unsafe fn new_unchecked(num: u32) -> Self {
        debug_assert!(num <= 0xFFFFFF);
        let buf = num.to_be_bytes();
        Self(*buf.last_chunk::<3>().unwrap())
    }

    /// Creates a big-endian [`uint24`] from big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::uint24;
    ///
    /// assert_eq!(uint24::from_be_bytes([0x00, 0x00, 0x07]).get(), 7);
    /// assert_eq!(uint24::from_be_bytes([0x12, 0x34, 0x56]).get(), 0x00123456);
    /// ```
    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self(bytes)
    }
    /// Gets this [`uint24`]'s big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::uint24;
    ///
    /// assert_eq!(uint24::new(7).unwrap().to_be_bytes(), [0x00, 0x00, 0x07]);
    /// assert_eq!(uint24::new(0x00123456).unwrap().to_be_bytes(), [0x12, 0x34, 0x56]);
    /// ```
    pub const fn to_be_bytes(self) -> [u8; 3] {
        self.0
    }

    /// Gets the value of this [`uint24`] as [`u32`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::uint24;
    ///
    /// assert_eq!(uint24::new(1234).unwrap().get(), 1234);
    /// assert_eq!(uint24::new(15_000_000).unwrap().get(), 15_000_000);
    /// ```
    pub const fn get(&self) -> u32 {
        let mut buf = [0; 4];
        buf[1..].copy_from_slice(&self.0);
        u32::from_be_bytes(buf)
    }
}

impl_fmt_with! {
    Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp:
    |this: &uint24, f| this.get().fmt(f)
}

// TODO: When [u8; 3]'s Default is constified, replace this impl with #[derive_const]
#[allow(clippy::derivable_impls)]
const impl Default for uint24 {
    fn default() -> Self {
        Self([0; 3])
    }
}

const impl PartialEq<u32> for uint24 {
    fn eq(&self, other: &u32) -> bool {
        self.get().eq(other)
    }
}
const impl PartialOrd<u32> for uint24 {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.get().cmp(other))
    }
}

const impl std::str::FromStr for uint24 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).or(Err(())).and_then(Self::try_from)
    }
}
const impl TryFrom<u32> for uint24 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
const impl From<uint24> for u32 {
    fn from(value: uint24) -> Self {
        value.get()
    }
}
