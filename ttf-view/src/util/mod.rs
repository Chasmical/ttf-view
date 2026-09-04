mod iter;

pub(crate) use iter::*;

/// Utility macro for formatting wrapper types, like uint24, F2DOT14, GlyphId
macro_rules! impl_fmt_with {
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
