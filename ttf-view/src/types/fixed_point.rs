use crate::util::impl_fmt_with;

macro_rules! impl_fixed_point_number {
    (
        $(#[$outer:meta])*
        $vis:vis struct $Name:ident(
            $int:ty as [u8; $bytes:literal];
            $integer_bits:literal | $fraction_bits:literal as $fp:ty
        );
        DENOM = $d_denom:literal;
        STEP = $d_step:literal;
        MIN = $d_min:literal;
        MAX = $d_max:literal;
        PRECISION = $d_precision:literal;
    ) => {
        #[doc = concat!("The [OpenType ", stringify!($Name), "][spec] type, a ")]
        #[doc = concat!(stringify!($integer_bits), ".", stringify!($fraction_bits), "-bit")]
        /// signed fixed-point type.
        ///
        /// [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#data-types
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq)]
        #[repr(transparent)]
        $vis struct $Name([u8; $bytes]);

        const _: () = {
            assert!(size_of::<$int>() == $bytes);
            assert!($integer_bits + $fraction_bits == <$int>::BITS);
        };

        impl $Name {
            const F_STEP: $fp = 1.0 / (1 << $fraction_bits) as $fp;
            const F_MIN: $fp = -(1 << ($integer_bits - 1)) as $fp;
            const F_MAX_EXCLUSIVE: $fp = (1 << ($integer_bits - 1)) as $fp;
            const F_MAX: $fp = Self::F_MAX_EXCLUSIVE - Self::F_STEP;

            #[doc = concat!("The difference between adjacent [`", stringify!($Name), "`] values.")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::STEP, ", stringify!($d_step), ");")]
            /// ```
            pub const STEP: Self = Self::new(Self::F_STEP).unwrap();

            /// The smallest value that can be represented by this type.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::MIN, ", stringify!($d_min), ");")]
            /// ```
            pub const MIN: Self = Self::new(Self::F_MIN).unwrap();
            /// The largest value that can be represented by this type.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::MAX, ", stringify!($d_max), ");")]
            /// ```
            pub const MAX: Self = Self::new(Self::F_MAX).unwrap();

            /// The amount of decimal places the type can accurately represent.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("# use ttf_view::types::", stringify!($Name), ";")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::PRECISION, ", stringify!($d_precision), ");")]
            /// ```
            pub const PRECISION: u32 = (Self::F_STEP.recip() as u32).ilog10();

            #[doc = concat!("Creates a [`", stringify!($Name), "`] from [`", stringify!($fp), "`].")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use ttf_view::types::", stringify!($Name), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(-0.125).unwrap(), -0.125);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(1.99609375).unwrap(), 1.99609375);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(123456.78), None);")]
            ///
            #[doc = concat!("// numbers are rounded towards the closest value representable by ", stringify!($Name))]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(0.5156247).unwrap(), 0.515625);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(0.5156250).unwrap(), 0.515625);")]
            #[doc = concat!("assert_eq!(", stringify!($Name), "::new(0.5156254).unwrap(), 0.515625);")]
            /// ```
            pub const fn new(num: $fp) -> Option<Self> {
                if matches!(num, Self::F_MIN..Self::F_MAX_EXCLUSIVE) {
                    Some(unsafe { Self::new_unchecked(num) })
                } else {
                    None
                }
            }
            #[doc = concat!("Creates a [`", stringify!($Name), "`] from [`", stringify!($fp), "`] without checks.")]
            ///
            /// TODO: # Examples
            pub const unsafe fn new_unchecked(num: $fp) -> Self {
                debug_assert!(matches!(num, Self::F_MIN..Self::F_MAX_EXCLUSIVE));

                // Note: No need to worry about 1.9999999 wrapping to -2, because `as` here
                // converts in a saturating way (even Infinity becomes 0xFFFF through `as`).
                Self(((num / Self::F_STEP).round() as $int).to_be_bytes())
            }

            #[doc = concat!("Creates a [`", stringify!($Name), "`] from big-endian bytes.")]
            ///
            /// TODO: # Examples
            pub const fn from_be_bytes(bytes: [u8; $bytes]) -> Self {
                Self(bytes)
            }
            #[doc = concat!("Gets this [`", stringify!($Name), "`]'s big-endian bytes.")]
            ///
            /// TODO: # Examples
            pub const fn to_be_bytes(self) -> [u8; $bytes] {
                self.0
            }

            #[doc = concat!("Returns this [`", stringify!($Name), "`] fraction's numerator")]
            #[doc = concat!("(`", stringify!($Name), "` represented as <math><mfrac><mi>numerator</mi><mn>", stringify!($d_denom), "</mn></mfrac></math>).")]
            ///
            /// TODO: # Examples
            pub const fn frac_num(&self) -> $int {
                <$int>::from_be_bytes(self.0)
            }
            #[doc = concat!("Returns this [`", stringify!($Name), "`]'s value as [`", stringify!($fp), "`].")]
            ///
            /// TODO: # Examples
            pub const fn get(&self) -> $fp {
                self.frac_num() as $fp * Self::F_STEP
            }
            #[doc = concat!("Rounds this [`", stringify!($Name), "`]'s value to [`PRECISION`][Self::PRECISION] decimal places.")]
            ///
            /// TODO: # Examples
            pub const fn round_to_precision(&self) -> $fp {
                const SCALE: $fp = 10u32.pow($Name::PRECISION) as $fp;
                (self.get() * SCALE).round() / SCALE
            }
        }

        impl_fmt_with! {
            Debug, Display, LowerExp, UpperExp:
            |x: &$Name| x.get()
        }

        const impl PartialOrd for $Name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        const impl Ord for $Name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.frac_num().cmp(&other.frac_num())
            }
        }

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

        impl std::str::FromStr for $Name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$fp>::from_str(s).or(Err(())).and_then($Name::try_from)
            }
        }
        const impl TryFrom<$fp> for $Name {
            type Error = ();
            fn try_from(value: $fp) -> Result<$Name, Self::Error> {
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
    pub struct Fixed(i32 as [u8; 4]; 16|16 as f64);
    DENOM = 65536;
    STEP = 0.0000152587890625;
    MIN = -32768.0;
    MAX = 32767.99998474121;
    PRECISION = 4;
}
impl_fixed_point_number! {
    pub struct F2DOT14(i16 as [u8; 2]; 2|14 as f32);
    DENOM = 16384;
    STEP = 0.000061035156;
    MIN = -2.0;
    MAX = 1.999939;
    PRECISION = 4;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precision() {
        // Precision tests for Fixed (font_revision)
        let f = Fixed::from_be_bytes(0x0001999A_u32.to_be_bytes());
        assert_eq!(format!("{}", f), "1.600006103515625");
        assert_eq!(format!("{}", f.round_to_precision()), "1.6");

        assert_eq!(format!("{:.0}", f), "2");
        assert_eq!(format!("{:.1}", f), "1.6");
        assert_eq!(format!("{:.3}", f), "1.600");
        assert_eq!(format!("{:.7}", f), "1.6000061");

        // Ensure the upper boundary is correctly rounded down
        assert_eq!(Fixed::new(32767.999999999996).unwrap().to_be_bytes(), [0x7F, 0xFF, 0xFF, 0xFF]);
        assert_eq!(Fixed::new(32768.0), None);
        assert_eq!(F2DOT14::new(1.9999999).unwrap().to_be_bytes(), [0x7F, 0xFF]);
        assert_eq!(F2DOT14::new(2.0), None);

        // Ensure the lower boundary is at an integer
        assert_eq!(Fixed::new(-32768.0).unwrap().to_be_bytes(), [0x80, 0x00, 0x00, 0x00]);
        assert_eq!(Fixed::new(-32768.00000000001), None);
        assert_eq!(F2DOT14::new(-2.0).unwrap().to_be_bytes(), [0x80, 0x00]);
        assert_eq!(F2DOT14::new(-2.0000002), None);
    }

    #[test]
    fn fixed() {
        // Check f64 bounds used in parameter validation
        assert_eq!(Fixed::F_MIN, -32768.0);
        assert_eq!(Fixed::F_MAX_EXCLUSIVE, 32768.0);
        assert_eq!(Fixed::F_MAX, 32767.99998474121);

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
            let real = Fixed::new(fp).unwrap().frac_num() as u32;
            assert_eq!(real, raw, "{real:#X} != {raw:#X} ({fp})");

            let real = Fixed::new(fp).unwrap().get();
            let diff = (real - fp).abs();
            assert!(diff <= 0.1 * Fixed::F_STEP, "{real} != {fp} (Δ={diff})");
        }
    }

    #[test]
    fn f2dot14() {
        // Check f32 bounds used in parameter validation
        assert_eq!(F2DOT14::F_MIN, -2.0);
        assert_eq!(F2DOT14::F_MAX_EXCLUSIVE, 2.0);
        assert_eq!(F2DOT14::F_MAX, 1.999939);

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
            let real = F2DOT14::new(fp).unwrap().frac_num() as u16;
            assert_eq!(real, raw, "{real:#X} != {raw:#X} ({fp})");

            let real = F2DOT14::new(fp).unwrap().get();
            let diff = (real - fp).abs();
            assert!(diff <= 0.1 * F2DOT14::F_STEP, "{real} != {fp} (Δ={diff})");
        }
    }
}
