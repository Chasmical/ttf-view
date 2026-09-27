use crate::{
    tables::{cmap::GlyphId, glyf::Glyph}, types::{F2DOT14, uint16},
};

#[repr(C)]
pub struct CompositeGlyph {
    header: Glyph,
}

const impl std::ops::Deref for CompositeGlyph {
    type Target = Glyph;
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

#[repr(C)]
pub struct ComponentGlyph {
    _exhaustive_but_dont_instantiate: (),
    pub flags: ComponentFlags,
    pub glyph_index: uint16,
    // : if ( flags & ARG_1_AND_2_ARE_WORDS ) {
    // :     (int16 or FWORD) argument1;
    // :     (int16 or FWORD) argument2;
    // : } else {
    // :     uint16 arg1and2; /* (arg1 << 8) | arg2 */
    // : }
    // : if ( flags & WE_HAVE_A_SCALE ) {
    // :     F2DOT14  scale;
    // : } else if ( flags & WE_HAVE_AN_X_AND_Y_SCALE ) {
    // :     F2DOT14  xscale;
    // :     F2DOT14  yscale;
    // : } else if ( flags & WE_HAVE_A_TWO_BY_TWO ) {
    // :     F2DOT14  xscale;
    // :     F2DOT14  scale01;
    // :     F2DOT14  scale10;
    // :     F2DOT14  yscale;
    // : }
}

struct ComponentInfo {
    pub flags: ComponentFlags,
    pub glyph_index: GlyphId,
    pub argument1: i16,
    pub argument2: i16,
    pub scale_xx: F2DOT14,
    pub scale_xy: F2DOT14,
    pub scale_yx: F2DOT14,
    pub scale_yy: F2DOT14,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ComponentFlags: u16 {
        const ARG_1_AND_2_ARE_WORDS = 0x0001;
        const ARGS_ARE_XY_VALUES = 0x0002;
        const ROUND_XY_TO_GRID = 0x0004;
        const WE_HAVE_A_SCALE = 0x0008;
        const MORE_COMPONENTS = 0x0020;
        const WE_HAVE_AN_X_AND_Y_SCALE = 0x0040;
        const WE_HAVE_A_TWO_BY_TWO = 0x0080;
        const WE_HAVE_INSTRUCTIONS = 0x0100;
        const USE_MY_METRICS = 0x0200;
        const OVERLAP_COMPOUND = 0x0400;
        const SCALED_COMPONENT_OFFSET = 0x0800;
        const UNSCALED_COMPONENT_OFFSET = 0x1000;
        const RESERVED = 0xE010;
    }
}
