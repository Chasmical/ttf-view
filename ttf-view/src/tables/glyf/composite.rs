use crate::{
    tables::glyf::Glyph,
    types::{BigEndian, F2DOT14, int16, uint16},
};

#[repr(C)]
pub struct CompositeGlyph {
    base: Glyph,
}

const impl std::ops::Deref for CompositeGlyph {
    type Target = Glyph;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[repr(C)]
pub struct Component {
    pub flags: ComponentFlags,
    pub glyph_index: uint16,
    data: [uint16; 0],
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

impl Component {
    pub const fn arguments(&self) -> (i16, i16) {
        if self.flags.intersects(ComponentFlags::ARG_1_AND_2_ARE_WORDS) {
            let [a1, a2] = unsafe { *self.data.as_ptr().cast::<[int16; 2]>() };
            (a1.get(), a2.get())
        } else {
            let [a1, a2] = unsafe { *self.data.as_ptr().cast::<[u8; 2]>() };
            (a1.into(), a2.into())
        }
    }
    pub const fn scale(&self) -> Option<(F2DOT14, F2DOT14, F2DOT14, F2DOT14)> {
        let long_args = self.flags.intersects(ComponentFlags::ARG_1_AND_2_ARE_WORDS);
        let args_len = if long_args { 4 } else { 2 };
        let ptr = unsafe { self.data.as_ptr().byte_add(args_len) };

        Some(unsafe {
            if self.flags.intersects(ComponentFlags::WE_HAVE_A_SCALE) {
                let scale = (&*ptr.cast::<BigEndian<F2DOT14>>()).get();
                (scale, F2DOT14::default(), F2DOT14::default(), scale)
            } else if self.flags.intersects(ComponentFlags::WE_HAVE_AN_X_AND_Y_SCALE) {
                let [xx, yy] = *ptr.cast::<[BigEndian<F2DOT14>; 2]>();
                (xx.get(), F2DOT14::default(), F2DOT14::default(), yy.get())
            } else if self.flags.intersects(ComponentFlags::WE_HAVE_A_TWO_BY_TWO) {
                let [xx, xy, yx, yy] = *ptr.cast::<[BigEndian<F2DOT14>; 4]>();
                (xx.get(), xy.get(), yx.get(), yy.get())
            } else {
                return None;
            }
        })
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ComponentFlags: u16 {
        const ARG_1_AND_2_ARE_WORDS = 0x0001_u16.to_be();
        const ARGS_ARE_XY_VALUES = 0x0002_u16.to_be();
        const ROUND_XY_TO_GRID = 0x0004_u16.to_be();
        const WE_HAVE_A_SCALE = 0x0008_u16.to_be();
        const MORE_COMPONENTS = 0x0020_u16.to_be();
        const WE_HAVE_AN_X_AND_Y_SCALE = 0x0040_u16.to_be();
        const WE_HAVE_A_TWO_BY_TWO = 0x0080_u16.to_be();
        const WE_HAVE_INSTRUCTIONS = 0x0100_u16.to_be();
        const USE_MY_METRICS = 0x0200_u16.to_be();
        const OVERLAP_COMPOUND = 0x0400_u16.to_be();
        const SCALED_COMPONENT_OFFSET = 0x0800_u16.to_be();
        const UNSCALED_COMPONENT_OFFSET = 0x1000_u16.to_be();
        const RESERVED = 0xE010_u16.to_be();
    }
}
