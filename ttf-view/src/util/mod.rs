mod custom_iter;
mod display_buffer;
mod packed_dual_iter;

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
