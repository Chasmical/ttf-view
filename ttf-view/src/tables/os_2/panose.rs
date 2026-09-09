#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Panose {
    pub b_family_type: u8,
    pub b_serif_style: u8,
    pub b_weight: u8,
    pub b_proportion: u8,
    pub b_contrast: u8,
    pub b_stroke_variation: u8,
    pub b_arm_style: u8,
    pub b_letterform: u8,
    pub b_midline: u8,
    pub b_x_height: u8,
}

// TODO: Panose needs a bunch of useful enums and structs here

impl std::fmt::Debug for Panose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Panose")
            .field("b_family_type", &self.b_family_type)
            .field("b_serif_style", &self.b_serif_style)
            .field("b_weight", &self.b_weight)
            .field("b_proportion", &self.b_proportion)
            .field("b_contrast", &self.b_contrast)
            .field("b_stroke_variation", &self.b_stroke_variation)
            .field("b_arm_style", &self.b_arm_style)
            .field("b_letterform", &self.b_letterform)
            .field("b_midline", &self.b_midline)
            .field("b_x_height", &self.b_x_height)
            .finish()
    }
}
