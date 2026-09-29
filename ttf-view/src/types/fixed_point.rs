use crate::util::impl_fmt_with;

macro_rules! impl_fixed_point_number {
    (
        $(#[$outer:meta])*
        $vis:vis struct $Name:ident(
            $int:ty as [u8; $bytes:literal];
            $integer_bits:literal | $fraction_bits:literal as $fp:ty, $wide:ty
        );
        DENOM = $denom:literal;
        STEP = $step:literal;
        MIN = $min:literal;
        MAX = $max:literal;
    ) => {
        #[doc = concat!("The [OpenType ", stringify!($Name), "][spec] type, a ")]
        #[doc = concat!(stringify!($integer_bits), ".", stringify!($fraction_bits), "-bit")]
        /// signed fixed-point type.
        ///
        /// [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#data-types
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        $vis struct $Name($int);

        const _: () = {
            assert!(size_of::<$int>() == $bytes);
            assert!($integer_bits + $fraction_bits == <$int>::BITS);
        };

        impl $Name {
            const F_STEP: $fp = 1.0 / (1 << $fraction_bits) as $fp;
            const F_MIN: $fp = -(1 << ($integer_bits - 1)) as $fp;
            const F_MAX_EXCLUSIVE: $fp = (1 << ($integer_bits - 1)) as $fp;

            #[doc = concat!("The difference between adjacent [`", stringify!($Name), "`] values.")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::STEP, ", stringify!($step), ");")]
            /// ```
            pub const STEP: Self = Self::new(Self::F_STEP).unwrap();

            /// The smallest value that can be represented by this type.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::MIN, ", stringify!($min), ");")]
            /// ```
            pub const MIN: Self = Self::new(Self::F_MIN).unwrap();
            /// The largest value that can be represented by this type.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::MAX, ", stringify!($max), ");")]
            /// ```
            pub const MAX: Self = Self::new(Self::F_MAX).unwrap();

            /// The value zero.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::ZERO, 0.0);")]
            /// ```
            pub const ZERO: Self = Self::new(0.0).unwrap();
            /// The value one.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::ONE, 1.0);")]
            /// ```
            pub const ONE: Self = Self::new(1.0).unwrap();

            /// This type's integer fraction's denominator.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::DENOM, ", stringify!($denom), ");")]
            /// ```
            pub const DENOM: $int = Self::ONE.0;

            #[doc = concat!("Creates a [`", stringify!($Name), "`] from [`", stringify!($fp), "`],")]
            #[doc = concat!("returning `None` if the value is out of `", stringify!($Name), "`'s range.")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ttf_view::types::", stringify!($Name), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(-0.125).unwrap(), -0.125);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(1.99609375).unwrap(), 1.99609375);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(-34000.0), None);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(34000.0), None);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(", stringify!($fp), "::NAN), None);")]
            ///
            #[doc = concat!("// numbers are rounded towards the closest value representable by ", stringify!($Name))]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(0.5156247).unwrap(), 0.515625);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(0.5156250).unwrap(), 0.515625);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(0.5156254).unwrap(), 0.515625);")]
            /// ```
            pub const fn new(num: $fp) -> Option<Self> {
                if matches!(num, Self::F_MIN..Self::F_MAX_EXCLUSIVE) {
                    Some(Self::new_saturating(num))
                } else {
                    None
                }
            }
            #[doc = concat!("Creates a [`", stringify!($Name), "`] from [`", stringify!($fp), "`],")]
            /// saturating at the numeric bounds, and mapping NaN to 0.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ttf_view::types::", stringify!($Name), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(-0.125), -0.125);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(1.99609375), 1.99609375);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(-34000.0), ", $min, ");")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(34000.0), ", $max, ");")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(", stringify!($fp), "::NAN), 0.0);")]
            ///
            #[doc = concat!("// numbers are rounded towards the closest value representable by ", stringify!($Name))]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(0.5156247), 0.515625);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(0.5156250), 0.515625);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new_saturating(0.5156254), 0.515625);")]
            /// ```
            pub const fn new_saturating(num: $fp) -> Self {
                // Note: No need to worry about 1.9999999 wrapping to -2, because `as` here
                // converts in a saturating way (even Infinity becomes 0x7FFF through `as`).
                Self((num / Self::F_STEP).round() as $int)
            }

            #[doc = concat!("Creates a [`", stringify!($Name), "`] from big-endian bytes.")]
            pub const fn from_be_bytes(bytes: [u8; $bytes]) -> Self {
                Self(<$int>::from_be_bytes(bytes))
            }
            #[doc = concat!("Gets this [`", stringify!($Name), "`]'s big-endian bytes.")]
            pub const fn to_be_bytes(self) -> [u8; $bytes] {
                self.0.to_be_bytes()
            }

            #[doc = concat!("Creates a [`", stringify!($Name), "`] from its integer fraction's numerator")]
            #[doc = concat!("(`", stringify!($Name), "` represented as <span class=\"hidden\">`numerator/", stringify!($denom), "`</span><math><mfrac><mi>numerator</mi><mn>", stringify!($denom), "</mn></mfrac></math>).")]
            pub const fn from_frac_num(numerator: $int) -> Self {
                Self(numerator)
            }
            #[doc = concat!("Returns this [`", stringify!($Name), "`]'s integer fraction's numerator")]
            #[doc = concat!("(`", stringify!($Name), "` represented as <span class=\"hidden\">`numerator/", stringify!($denom), "`</span><math><mfrac><mi>numerator</mi><mn>", stringify!($denom), "</mn></mfrac></math>).")]
            pub const fn frac_num(self) -> $int {
                self.0
            }

            #[doc = concat!("Returns this [`", stringify!($Name), "`]'s value as [`", stringify!($fp), "`].")]
            pub const fn get(self) -> $fp {
                self.frac_num() as $fp * Self::F_STEP
            }

            /// Widening numerator multiplication: x/d * y/d = xy/dd.
            ///
            /// Don't forget to divide the wide numerator by [`DENOM`][Self::DENOM] before casting
            /// it back to [`Self`] with [`from_frac_num`][Self::from_frac_num].
            pub(crate) const fn wmul(self, rhs: Self) -> $wide {
                self.0.widening_mul(rhs.0)
            }

            pub const fn wrapping_neg(self) -> Self {
                Self(self.0.wrapping_neg())
            }
            pub const fn wrapping_add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }
            pub const fn wrapping_mul(self, rhs: Self) -> Self {
                Self((self.wmul(rhs) / $denom) as $int)
            }
            /// # Panics
            ///
            /// This function panics if `rhs == 0`.
            pub const fn wrapping_div(self, rhs: Self) -> Self {
                Self((self.wmul(Self::ONE) / rhs.0 as $wide) as $int)
            }
            /// # Panics
            ///
            /// This function panics if `rhs == 0`.
            pub const fn wrapping_rem(self, rhs: Self) -> Self {
                Self(self.0.wrapping_rem(rhs.0))
            }

            pub const fn saturating_neg(self) -> Self {
                Self(self.0.saturating_neg())
            }
            pub const fn saturating_add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }
            pub const fn saturating_mul(self, rhs: Self) -> Self {
                Self((self.wmul(rhs) / $denom).saturating_cast())
            }
            /// # Panics
            ///
            /// This function panics if `rhs == 0`.
            pub const fn saturating_div(self, rhs: Self) -> Self {
                Self((self.wmul(Self::ONE) / rhs.0 as $wide).saturating_cast())
            }

            pub const fn checked_neg(self) -> Option<Self> {
                self.0.checked_neg().map(Self)
            }
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                self.0.checked_add(rhs.0).map(Self)
            }
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                self.0.checked_sub(rhs.0).map(Self)
            }
            pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
                (self.wmul(rhs) / $denom).checked_cast().map(Self)
            }
            pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                (self.wmul(Self::ONE).checked_div(rhs.0 as $wide)?).checked_cast().map(Self)
            }
            pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                self.0.checked_rem(rhs.0).map(Self)
            }

            /// Wrapping product sum operation (ab+cd+p)
            pub(crate) const fn wrapping_maddp(a: Self, b: Self, c: Self, d: Self, p: Self) -> Self {
                let sum = a.wmul(b).wrapping_add(c.wmul(d)) / $denom;
                Self(sum.wrapping_add(p.0 as $wide) as $int)
            }
            /// Saturating product sum operation (ab+cd+p)
            pub(crate) const fn saturating_maddp(a: Self, b: Self, c: Self, d: Self, p: Self) -> Self {
                let sum = a.wmul(b).saturating_add(c.wmul(d)) / $denom;
                Self(sum.saturating_add(p.0 as $wide).saturating_cast())
            }
            /// Checked product sum operation (ab+cd+p)
            pub(crate) const fn checked_maddp(a: Self, b: Self, c: Self, d: Self, p: Self) -> Option<Self> {
                let sum = a.wmul(b).checked_add(c.wmul(d))? / $denom;
                Some(Self(sum.checked_add(p.0 as $wide)?.try_into().ok()?))
            }
        }

        /// Performs negation `-` (panics on overflow in debug configuration).
        const impl std::ops::Neg for $Name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                #[cfg(debug_assertions)]
                { self.checked_neg().expect("attempt to negate with overflow") }
                #[cfg(not(debug_assertions))]
                { self.wrapping_neg() }
            }
        }
        /// Performs addition `+` (panics on overflow in debug configuration).
        const impl std::ops::Add for $Name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #[cfg(debug_assertions)]
                { self.checked_add(rhs).expect("attempt to add with overflow") }
                #[cfg(not(debug_assertions))]
                { self.wrapping_add(rhs) }
            }
        }
        /// Performs subtraction `-` (panics on overflow in debug configuration).
        const impl std::ops::Sub for $Name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #[cfg(debug_assertions)]
                { self.checked_sub(rhs).expect("attempt to subtract with overflow") }
                #[cfg(not(debug_assertions))]
                { self.wrapping_sub(rhs) }
            }
        }
        /// Performs multiplication `*` (panics on overflow in debug configuration).
        const impl std::ops::Mul for $Name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                #[cfg(debug_assertions)]
                { self.checked_mul(rhs).expect("attempt to multiply with overflow") }
                #[cfg(not(debug_assertions))]
                { self.wrapping_mul(rhs) }
            }
        }
        /// Performs division `/` (panics on overflow in debug configuration).
        ///
        /// # Panics
        ///
        /// This operation will panic if `rhs == 0`.
        const impl std::ops::Div for $Name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                #[cfg(debug_assertions)]
                { self.checked_div(rhs).expect("attempt to divide by zero or with overflow") }
                #[cfg(not(debug_assertions))]
                { self.wrapping_div(rhs) }
            }
        }
        /// Performs remainder operation `%` (panics on overflow in debug configuration).
        ///
        /// # Panics
        ///
        /// This operation will panic if `rhs == 0`.
        const impl std::ops::Rem for $Name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                #[cfg(debug_assertions)]
                { self.checked_rem(rhs).expect("attempt to calculate the remainder with a divisor of zero or with overflow") }
                #[cfg(not(debug_assertions))]
                { self.wrapping_rem(rhs) }
            }
        }

        impl_fmt_with! { Debug, Display, LowerExp, UpperExp: |x: &$Name| x.get() }
        impl_fmt_with! { Binary, LowerHex, UpperHex: |x: &$Name| x.0 }

        const impl PartialEq<$fp> for $Name {
            fn eq(&self, other: &$fp) -> bool {
                self.get().eq(other)
            }
        }
        const impl PartialOrd<$fp> for $Name {
            fn partial_cmp(&self, other: &$fp) -> Option<std::cmp::Ordering> {
                self.get().partial_cmp(other)
            }
        }

        const impl std::ops::Mul<$fp> for $Name {
            type Output = $fp;
            fn mul(self, other: $fp) -> Self::Output {
                self.get().mul(other)
            }
        }
        const impl std::ops::Div<$fp> for $Name {
            type Output = $fp;
            fn div(self, other: $fp) -> Self::Output {
                self.get().div(other)
            }
        }

        impl std::str::FromStr for $Name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$fp>::from_str(s).or(Err(())).and_then(Self::try_from)
            }
        }
        const impl TryFrom<$fp> for $Name {
            type Error = ();
            fn try_from(value: $fp) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(())
            }
        }
        const impl From<$Name> for $fp {
            fn from(value: $Name) -> Self {
                value.get()
            }
        }
    }
}

// The constants specified here are re-calculated in the macro and then validated in doc-tests.
impl_fixed_point_number! {
    pub struct Fixed(i32 as [u8; 4]; 16|16 as f64, i64);
    DENOM = 65536;
    STEP = 0.0000152587890625;
    MIN = -32768.0;
    MAX = 32767.99998474121;
}
impl_fixed_point_number! {
    pub struct F2Dot14(i16 as [u8; 2]; 2|14 as f32, i32);
    DENOM = 16384;
    STEP = 0.000061035156;
    MIN = -2.0;
    MAX = 1.999939;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed() {
        // Check the rounding at boundaries
        assert_eq!(Fixed::new(32767.999999999996), Some(Fixed::MAX));
        assert_eq!(Fixed::new(32768.0), None);
        assert_eq!(Fixed::new(-32768.0), Some(Fixed::MIN));
        assert_eq!(Fixed::new(-32768.00000000001), None);

        // Test a bunch of sample numbers
        let nums: [(u32, f64); _] = [
            (0x7FFF_FFFF, 32767.999985),
            (0x7FFF_FF00, 32767.996094),
            (0x7FFF_2000, 32767.125000),
            (0x7FFF_0000, 32767.000000),
            (0x0040_0100, 64.003906),
            (0x0040_0000, 64.000000),
            (0x0001_0000, 1.000000),
            (0x0000_0001, 0.000015),
            (0x0000_0000, 0.000000),
            (0xFFFF_0000, -1.000000),
            (0xFFBF_FF00, -64.003906),
            (0x8000_0000, -32768.000000),
        ];

        for (raw, fp) in nums {
            let real = Fixed::new(fp).unwrap();
            assert_eq!(real.0 as u32, raw, "{real:#X} != {raw:#X} ({fp})");

            let diff = (real.get() - fp).abs();
            assert!(diff <= 0.1 * Fixed::F_STEP, "{real} != {fp} (Δ={diff})");
        }
    }

    #[test]
    fn f2dot14() {
        // Check the rounding at boundaries
        assert_eq!(F2Dot14::new(1.9999999), Some(F2Dot14::MAX));
        assert_eq!(F2Dot14::new(2.0), None);
        assert_eq!(F2Dot14::new(-2.0), Some(F2Dot14::MIN));
        assert_eq!(F2Dot14::new(-2.0000002), None);

        // Test a bunch of sample numbers
        let nums: [(u16, f32); _] = [
            (0x7FFF, 1.999939),
            (0x7000, 1.750000),
            (0x0085, 0.008118),
            (0x0002, 0.000122),
            (0x0001, 0.000061),
            (0x0000, 0.000000),
            (0xFFFF, -0.000061),
            (0xFFFE, -0.000122),
            (0xFF7B, -0.008118),
            (0x8000, -2.000000),
        ];

        for (raw, fp) in nums {
            let real = F2Dot14::new(fp).unwrap();
            assert_eq!(real.0 as u16, raw, "{real:#X} != {raw:#X} ({fp})");

            let diff = (real.get() - fp).abs();
            assert!(diff <= 0.1 * F2Dot14::F_STEP, "{real} != {fp} (Δ={diff})");
        }
    }
}
