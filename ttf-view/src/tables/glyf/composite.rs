use crate::{
    tables::glyf::Glyph,
    types::{Affine2x2, BigEndian, F2Dot14, uint16},
};

#[repr(C)]
pub struct CompositeGlyph {
    base: Glyph,
    first_component: Component,
}

const impl std::ops::Deref for CompositeGlyph {
    type Target = Glyph;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[repr(C)]
pub struct Component {
    pub flags: uint16,
    pub glyph_index: uint16,
    data: [uint16; 0],
    // : if ( flags & ARG_1_AND_2_ARE_WORDS ) {
    // :     (int16 or FWORD) argument1;
    // :     (int16 or FWORD) argument2;
    // : } else {
    // :     uint16 arg1and2; /* (arg1 << 8) | arg2 */
    // : }
    // : if ( flags & WE_HAVE_A_SCALE ) {
    // :     F2Dot14  scale;
    // : } else if ( flags & WE_HAVE_AN_X_AND_Y_SCALE ) {
    // :     F2Dot14  xscale;
    // :     F2Dot14  yscale;
    // : } else if ( flags & WE_HAVE_A_TWO_BY_TWO ) {
    // :     F2Dot14  xscale;
    // :     F2Dot14  scale01;
    // :     F2Dot14  scale10;
    // :     F2Dot14  yscale;
    // : }
}

impl CompositeGlyph {
    pub const fn components(&self) -> ComponentsIter<'_> {
        ComponentsIter::new(&self.first_component)
    }
}

impl Component {
    pub const fn flags(&self) -> ComponentFlags {
        ComponentFlags::from_bits_retain(self.flags.get())
    }
    pub const fn arguments(&self) -> (i32, i32) {
        if self.flags().intersects(ComponentFlags::ARGS_ARE_LONG) {
            let [a1, a2] = unsafe { *self.data.as_ptr().cast::<[uint16; 2]>() };
            let (a1, a2) = (a1.get(), a2.get());

            if self.flags().intersects(ComponentFlags::ARGS_ARE_SIGNED_COORDS) {
                (a1.cast_signed() as i32, a2.cast_signed() as i32)
            } else {
                (a1 as i32, a2 as i32)
            }
        } else {
            let [a1, a2] = unsafe { *self.data.as_ptr().cast::<[u8; 2]>() };

            if self.flags().intersects(ComponentFlags::ARGS_ARE_SIGNED_COORDS) {
                (a1.cast_signed() as i32, a2.cast_signed() as i32)
            } else {
                (a1 as i32, a2 as i32)
            }
        }
    }
    pub const fn transform(&self) -> Affine2x2 {
        let ptr = unsafe { self.data.as_ptr().byte_add(self.flags().args_size() as _) };

        unsafe {
            if self.flags().intersects(ComponentFlags::SCALE_IS_SIMPLE) {
                let scale = (&*ptr.cast::<BigEndian<F2Dot14>>()).get();
                Affine2x2::scale(scale)
            } else if self.flags().intersects(ComponentFlags::SCALE_IS_XY) {
                let [x, y] = *ptr.cast::<[BigEndian<F2Dot14>; 2]>();
                Affine2x2::scale_xy(x.get(), y.get())
            } else if self.flags().intersects(ComponentFlags::SCALE_IS_FULL) {
                (&*ptr.cast::<BigEndian<Affine2x2>>()).get()
            } else {
                Affine2x2::IDENTITY
            }
        }
    }

    pub const fn next_component(&self) -> Option<&Self> {
        if !self.flags().intersects(ComponentFlags::MORE_COMPONENTS) {
            return None;
        }
        let dyn_size = self.flags().args_and_transform_size() as usize;
        Some(unsafe { &*self.data.as_ptr().byte_add(dyn_size).cast::<Self>() })
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct ComponentFlags: u16 {
        const ARGS_ARE_LONG = 0x0001; // ARG_1_AND_2_ARE_WORDS
        const ARGS_ARE_SIGNED_COORDS = 0x0002; // ARGS_ARE_XY_VALUES
        const ROUND_COORDS_TO_GRID = 0x0004; // ROUND_XY_TO_GRID
        const SCALE_IS_SIMPLE = 0x0008; // WE_HAVE_A_SCALE
        // const RESERVED = 0x0010;
        const MORE_COMPONENTS = 0x0020;
        const SCALE_IS_XY = 0x0040; // WE_HAVE_AN_X_AND_Y_SCALE
        const SCALE_IS_FULL = 0x0080; // WE_HAVE_A_TWO_BY_TWO
        const WE_HAVE_INSTRUCTIONS = 0x0100;
        const USE_MY_METRICS = 0x0200;
        const OVERLAP_COMPOUND = 0x0400;
        const SCALED_COMPONENT_OFFSET = 0x0800;
        const UNSCALED_COMPONENT_OFFSET = 0x1000;
        // const RESERVED = 0x2000;
        // const RESERVED = 0x4000;
        // const RESERVED = 0x8000;
    }
}

impl ComponentFlags {
    const fn args_size(&self) -> u8 {
        if self.intersects(Self::ARGS_ARE_LONG) { 4 } else { 2 }
    }
    const fn transform_size(&self) -> u8 {
        if self.intersects(ComponentFlags::SCALE_IS_SIMPLE) {
            2
        } else if self.intersects(ComponentFlags::SCALE_IS_XY) {
            4
        } else if self.intersects(ComponentFlags::SCALE_IS_FULL) {
            8
        } else {
            0
        }
    }
    pub(crate) const fn args_and_transform_size(&self) -> u8 {
        self.args_size() + self.transform_size()
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct ComponentsIter<'a>(Option<&'a Component>);

impl<'a> ComponentsIter<'a> {
    pub const fn new(first_component: &'a Component) -> Self {
        Self(Some(first_component))
    }
}

impl<'a> Iterator for ComponentsIter<'a> {
    type Item = &'a Component;
    fn next(&mut self) -> Option<Self::Item> {
        let this = self.0?;
        self.0 = this.next_component();
        Some(this)
    }
}
impl<'a> std::iter::FusedIterator for ComponentsIter<'a> {}
