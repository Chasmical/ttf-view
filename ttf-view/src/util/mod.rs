mod cursor;
mod custom_iter;
mod display_buffer;
mod packed_dual_iter;

pub(crate) use cursor::*;
pub(crate) use custom_iter::*;
pub(crate) use display_buffer::*;
pub(crate) use packed_dual_iter::*;

/// Utility macro for formatting wrapper types, like u24, F2Dot14, GlyphId
macro_rules! impl_fmt_with {
    ($($Trait:ident),*: |$arg:ident: &$Name:ty| $closure:expr) => {
        impl_fmt_with! { $($Trait),*: |$arg: &$Name, f| $closure.fmt(f) }
    };
    ($($Trait:ident),*: |$arg:ident: &$Name:ty, $f:ident| $closure:expr) => ($(
        impl std::fmt::$Trait for $Name {
            fn fmt(&self, $f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let $arg = self;
                $closure
            }
        }
    )*);
}
pub(crate) use impl_fmt_with;

// TODO: When saturating_cast is stabilized, replace all usages of this with it.
macro_rules! saturating_cast {
    (($value:expr) => $ty:ty) => {{
        let value = $value;
        if let Ok(cast) = value.try_into() {
            cast
        } else {
            if value < 0 { <$ty>::MIN } else { <$ty>::MAX }
        }
    }};
}
pub(crate) use saturating_cast;

// TODO: When Debug {field,entry}_with are stabilized, replace all usages of this with them.
// Although, maybe not all usages... this macro is pretty convenient, it's 9 chars shorter.
macro_rules! fmt_with {
    (|$f:ident| $closure:expr) => {
        &$crate::util::DebugOnce::new(|$f| $closure)
    };
    ($format:literal, $($tt:tt)*) => {
        fmt_with!(|f| write!(f, $format, $($tt)*))
    };
}
pub(crate) struct DebugOnce<F>(std::cell::Cell<Option<F>>);

impl<F: FnOnce(&mut std::fmt::Formatter) -> std::fmt::Result> DebugOnce<F> {
    pub fn new(f: F) -> Self {
        Self(std::cell::Cell::new(Some(f)))
    }
}
impl<F: FnOnce(&mut std::fmt::Formatter) -> std::fmt::Result> std::fmt::Debug for DebugOnce<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.take().unwrap()(f)
    }
}

pub(crate) use fmt_with;
