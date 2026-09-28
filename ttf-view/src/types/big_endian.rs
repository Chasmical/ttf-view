use crate::types::{
    Affine2x2, F2Dot14, Fixed, LongDateTime, Version16Dot16, int16, int32, int64, u24, uint16,
    uint32, uint64,
};

mod private {
    pub trait Sealed {}
}

pub const trait Primitive: private::Sealed + Copy {
    type BigEndian: Copy;
    fn from_big_endian(be: Self::BigEndian) -> Self;
    fn to_big_endian(ne: Self) -> Self::BigEndian;
}

macro_rules! impl_primitives {
    ($( $ne_ty:ty, $be_ty:ty, |$be:ident| $from_be:expr, |$ne:ident| $to_be:expr ; )*) => ($(
        impl private::Sealed for $ne_ty {}
        const impl Primitive for $ne_ty {
            type BigEndian = $be_ty;
            fn from_big_endian($be: Self::BigEndian) -> Self { $from_be }
            fn to_big_endian($ne: Self) -> Self::BigEndian { $to_be }
        }
        const impl From<BigEndian<$ne_ty>> for $ne_ty {
            fn from(value: BigEndian<$ne_ty>) -> Self {
                value.get()
            }
        }
    )*);
}
impl_primitives! {
    // TODO: Maybe we shouldn't be wrapping zerocopy's types here?
    i16, int16, |x| x.get(), |x| int16::new(x);
    i32, int32, |x| x.get(), |x| int32::new(x);
    i64, int64, |x| x.get(), |x| int64::new(x);
    u16, uint16, |x| x.get(), |x| uint16::new(x);
    u32, uint32, |x| x.get(), |x| uint32::new(x);
    u64, uint64, |x| x.get(), |x| uint64::new(x);

    Fixed, [u8; 4], |x| Fixed::from_be_bytes(x), |x| x.to_be_bytes();
    F2Dot14, [u8; 2], |x| F2Dot14::from_be_bytes(x), |x| x.to_be_bytes();
    LongDateTime, [u8; 8], |x| LongDateTime::from_be_bytes(x), |x| x.to_be_bytes();
    u24, [u8; 3], |x| u24::from_be_bytes(x), |x| x.to_be_bytes();
    Version16Dot16, [u8; 4], |x| unsafe { Version16Dot16::from_be_bytes_unchecked(x) },
    |x| x.to_be_bytes();
    Affine2x2, [u8; 8], |x| Affine2x2::from_be_bytes(x), |x| x.to_be_bytes();
}

#[derive(Copy)]
#[derive_const(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BigEndian<T: Primitive>(T::BigEndian);

impl<T: Primitive> BigEndian<T> {
    pub const fn new(value: T) -> Self
    where T: [const] Primitive {
        Self(T::to_big_endian(value))
    }
    pub const fn get(&self) -> T
    where T: [const] Primitive {
        T::from_big_endian(self.0)
    }
}

const impl<T: [const] Primitive> From<T> for BigEndian<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

// Extra impls for user-friendly big-endian types (zerocopy's, for example)
macro_rules! impl_extra_traits_for_big_endian {
    ($($ty:ty),* $(,)?) => ($(
        impl std::ops::Deref for BigEndian<$ty> {
            type Target = <$ty as Primitive>::BigEndian;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        const impl From<BigEndian<$ty>> for <$ty as Primitive>::BigEndian {
            fn from(value: BigEndian<$ty>) -> Self {
                value.0
            }
        }
        const impl From<<$ty as Primitive>::BigEndian> for BigEndian<$ty> {
            fn from(value: <$ty as Primitive>::BigEndian) -> Self {
                Self(value)
            }
        }
    )*);
}
impl_extra_traits_for_big_endian! {
    i16, i32, i64, u16, u32, u64,
}

macro_rules! impl_fmt_for_big_endian {
    ($($Trait:ident),* $(,)?) => ($(
        impl<T: Primitive + std::fmt::$Trait> std::fmt::$Trait for BigEndian<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.get().fmt(f)
            }
        }
    )*);
}
impl_fmt_for_big_endian! { Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp }
