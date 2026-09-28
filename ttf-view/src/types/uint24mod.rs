use crate::util::impl_fmt_with;

/// The 24-bit unsigned integer type.
///
/// **Important note:** Despite only representing a valid range of a 24-bit unsigned integers,
/// this type is actually backed by [`u32`] to ensure it's aligned well in the registers for
/// efficient math, copies and pretty much all operations in general.
///
/// ```
/// use ttf_view::types::{BigEndian, u24, uint24};
///
/// // u24 is backed by u32
/// assert_eq!(size_of::<u24>(), 4);
///
/// // big-endian types still have correct size (backed by [u8; 3])
/// assert_eq!(size_of::<uint24>(), 3);
/// assert_eq!(size_of::<BigEndian<u24>>(), 3);
/// ```
#[derive(Copy, Hash)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct u24(u32);

impl u24 {
    /// The size of this integer type in bits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ttf_view::types::u24;
    /// assert_eq!(u24::BITS, 24);
    /// ```
    pub const BITS: u32 = 24;
    /// The smallest value that can be represented by this integer type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ttf_view::types::u24;
    /// assert_eq!(u24::MIN, 0);
    /// ```
    pub const MIN: Self = Self(0x000000);
    /// The largest value that can be represented by this integer type (2<sup>24</sup> &minus; 1).
    ///
    /// # Examples
    ///
    /// ```
    /// # use ttf_view::types::u24;
    /// assert_eq!(u24::MAX, 16_777_215);
    /// ```
    pub const MAX: Self = Self(0xFFFFFF);

    /// Creates a [`u24`] from [`u32`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::u24;
    ///
    /// assert_eq!(u24::new(1234).unwrap(), 1234);
    /// assert_eq!(u24::new(15_000_000).unwrap(), 15_000_000);
    /// assert_eq!(u24::new(0xFFFFFF).unwrap(), 0xFFFFFF);
    /// assert_eq!(u24::new(17_000_000), None);
    /// assert_eq!(u24::new(3_999_000_000), None);
    /// ```
    pub const fn new(num: u32) -> Option<Self> {
        if num <= Self::MAX.0 { Some(unsafe { Self::new_unchecked(num) }) } else { None }
    }
    /// Creates a [`u24`] from [`u32`] without checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::u24;
    ///
    /// assert_eq!(unsafe { u24::new_unchecked(1234).get() }, 1234);
    /// assert_eq!(unsafe { u24::new_unchecked(15_000_000).get() }, 15_000_000);
    /// ```
    pub const unsafe fn new_unchecked(num: u32) -> Self {
        debug_assert!(num <= Self::MAX.0);
        Self(num)
    }

    /// Creates a [`u24`] from big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::u24;
    ///
    /// assert_eq!(u24::from_be_bytes([0x00, 0x00, 0x07]), 7);
    /// assert_eq!(u24::from_be_bytes([0x12, 0x34, 0x56]), 0x00123456);
    /// ```
    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        let [a, b, c] = bytes;
        Self(u32::from_be_bytes([0, a, b, c]))
    }
    /// Gets this [`u24`]'s big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::u24;
    ///
    /// assert_eq!(u24::new(7).unwrap().to_be_bytes(), [0x00, 0x00, 0x07]);
    /// assert_eq!(u24::new(0x00123456).unwrap().to_be_bytes(), [0x12, 0x34, 0x56]);
    /// ```
    pub const fn to_be_bytes(self) -> [u8; 3] {
        *self.0.to_be_bytes().last_chunk().unwrap()
    }

    /// Gets the value of this [`u24`] as [`u32`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::u24;
    ///
    /// assert_eq!(u24::new(1234).unwrap(), 1234);
    /// assert_eq!(u24::new(15_000_000).unwrap(), 15_000_000);
    /// ```
    pub const fn get(&self) -> u32 {
        self.0
    }

    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0) & Self::MAX.0)
    }
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0) & Self::MAX.0)
    }
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self(self.0.wrapping_mul(rhs.0) & Self::MAX.0)
    }
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self(self.0.wrapping_div(rhs.0) & Self::MAX.0)
    }

    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0).min(Self::MAX.0))
    }
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self(self.0.widening_mul(rhs.0).min(Self::MAX.0 as u64) as u32)
    }
    /// # Panics
    ///
    /// This function will panic if `rhs == 0`.
    pub const fn saturating_div(self, rhs: Self) -> Self {
        Self(self.0.saturating_div(rhs.0))
    }

    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.wrapping_add(rhs.0).try_into().ok()
    }
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.0.widening_mul(rhs.0).try_into().ok()
    }
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        self.0.checked_div(rhs.0).map(Self)
    }
}

/// Performs addition `+` (panics on overflow in debug configuration).
const impl std::ops::Add for u24 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        return self.checked_add(rhs).expect("attempt to add with overflow");
        #[cfg(not(debug_assertions))]
        return self.wrapping_add(rhs);
    }
}
/// Performs subtraction `-` (panics on overflow in debug configuration).
const impl std::ops::Sub for u24 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        return self.checked_sub(rhs).expect("attempt to subtract with overflow");
        #[cfg(not(debug_assertions))]
        return self.wrapping_sub(rhs);
    }
}
/// Performs multiplication `*` (panics on overflow in debug configuration).
const impl std::ops::Mul for u24 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        return self.checked_mul(rhs).expect("attempt to multiply with overflow");
        #[cfg(not(debug_assertions))]
        return self.wrapping_mul(rhs);
    }
}
/// Performs division `/` (panics on overflow in debug configuration).
///
/// # Panics
///
/// This operation will panic if `rhs == 0`.
const impl std::ops::Div for u24 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        return self.checked_div(rhs).expect("attempt to divide with overflow");
        #[cfg(not(debug_assertions))]
        return self.wrapping_div(rhs);
    }
}

impl_fmt_with! {
    Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp:
    |this: &u24| this.get()
}

const impl PartialEq<u32> for u24 {
    fn eq(&self, other: &u32) -> bool {
        self.get().eq(other)
    }
}
const impl PartialOrd<u32> for u24 {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.get().cmp(other))
    }
}

const impl std::str::FromStr for u24 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).or(Err(())).and_then(Self::try_from)
    }
}

const impl TryFrom<usize> for u24 {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().ok().and_then(Self::new).ok_or(())
    }
}
const impl TryFrom<u64> for u24 {
    type Error = ();
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        value.try_into().ok().and_then(Self::new).ok_or(())
    }
}
const impl TryFrom<u32> for u24 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
const impl From<u16> for u24 {
    fn from(value: u16) -> Self {
        Self(value as u32)
    }
}
const impl From<u8> for u24 {
    fn from(value: u8) -> Self {
        Self(value as u32)
    }
}

const impl TryFrom<u24> for usize {
    type Error = ();
    fn try_from(value: u24) -> Result<Self, Self::Error> {
        value.get().try_into().ok().ok_or(())
    }
}
const impl From<u24> for u64 {
    fn from(value: u24) -> Self {
        value.get() as u64
    }
}
const impl From<u24> for u32 {
    fn from(value: u24) -> Self {
        value.get()
    }
}
const impl TryFrom<u24> for u16 {
    type Error = ();
    fn try_from(value: u24) -> Result<Self, Self::Error> {
        value.get().try_into().ok().ok_or(())
    }
}
const impl TryFrom<u24> for u8 {
    type Error = ();
    fn try_from(value: u24) -> Result<Self, Self::Error> {
        value.get().try_into().ok().ok_or(())
    }
}
