use crate::types::{Tag, Version16Dot16, tags, uint16};

#[repr(C)]
pub struct MaxpV0 {
    _exhaustive_but_dont_instantiate: (),
    // version any:
    pub version: Version16Dot16,
}
#[repr(C)]
pub struct MaxpV05 {
    v0: MaxpV0,
    // version ≥ 0.5:
    pub num_glyphs: uint16,
}
#[repr(C)]
pub struct MaxpV1 {
    v05: MaxpV05,
    // version = 1.x:
    pub max_points: uint16,
    pub max_contours: uint16,
    pub max_composite_points: uint16,
    pub max_composite_contours: uint16,
    pub max_zones: uint16,
    pub max_twilight_points: uint16,
    pub max_storage: uint16,
    pub max_function_defs: uint16,
    pub max_instruction_defs: uint16,
    pub max_stack_elements: uint16,
    pub max_size_of_instructions: uint16,
    pub max_component_elements: uint16,
    pub max_component_depth: uint16,
}

const impl std::ops::Deref for MaxpV05 {
    type Target = MaxpV0;
    fn deref(&self) -> &Self::Target {
        &self.v0
    }
}
const impl std::ops::Deref for MaxpV1 {
    type Target = MaxpV05;
    fn deref(&self) -> &Self::Target {
        &self.v05
    }
}

impl super::RawTable for MaxpV0 {
    const TAG: Tag = tags::maxp;
}
impl<'a> super::Table<'a> for Maxp<'a> {
    const TAG: Tag = tags::maxp;
    fn in_directory(dir: &'a super::TableDirectory) -> Option<Self> {
        Some(Self { maxp: dir.table_raw()? })
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Maxp<'a> {
    maxp: &'a MaxpV0,
}

const impl<'a> std::ops::Deref for Maxp<'a> {
    type Target = &'a MaxpV0;
    fn deref(&self) -> &Self::Target {
        &self.maxp
    }
}

impl<'a> Maxp<'a> {
    pub const fn v05(&self) -> Option<&'a MaxpV05> {
        let (major, minor) = self.version.tuple();
        if major == 0 && minor >= 5 || major == 1 {
            Some(unsafe { std::mem::transmute::<&MaxpV0, &MaxpV05>(self.maxp) })
        } else {
            None
        }
    }
    pub const fn v1(&self) -> Option<&'a MaxpV1> {
        if self.version.major() == 1 {
            Some(unsafe { std::mem::transmute::<&MaxpV0, &MaxpV1>(self.maxp) })
        } else {
            None
        }
    }

    // version ≥ 0.5:
    pub const fn num_glyphs(&self) -> Option<uint16> {
        Some(self.v05()?.num_glyphs)
    }

    // version ≥ 1.0:
    pub const fn max_points(&self) -> Option<uint16> {
        Some(self.v1()?.max_points)
    }
    pub const fn max_contours(&self) -> Option<uint16> {
        Some(self.v1()?.max_contours)
    }
    pub const fn max_composite_points(&self) -> Option<uint16> {
        Some(self.v1()?.max_composite_points)
    }
    pub const fn max_composite_contours(&self) -> Option<uint16> {
        Some(self.v1()?.max_composite_contours)
    }
    pub const fn max_zones(&self) -> Option<uint16> {
        Some(self.v1()?.max_zones)
    }
    pub const fn max_twilight_points(&self) -> Option<uint16> {
        Some(self.v1()?.max_twilight_points)
    }
    pub const fn max_storage(&self) -> Option<uint16> {
        Some(self.v1()?.max_storage)
    }
    pub const fn max_function_defs(&self) -> Option<uint16> {
        Some(self.v1()?.max_function_defs)
    }
    pub const fn max_instruction_defs(&self) -> Option<uint16> {
        Some(self.v1()?.max_instruction_defs)
    }
    pub const fn max_stack_elements(&self) -> Option<uint16> {
        Some(self.v1()?.max_stack_elements)
    }
    pub const fn max_size_of_instructions(&self) -> Option<uint16> {
        Some(self.v1()?.max_size_of_instructions)
    }
    pub const fn max_component_elements(&self) -> Option<uint16> {
        Some(self.v1()?.max_component_elements)
    }
    pub const fn max_component_depth(&self) -> Option<uint16> {
        Some(self.v1()?.max_component_depth)
    }
}

impl std::fmt::Debug for Maxp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("MaxpTable");
        f.field("version", &self.version);

        if let Some(v05) = self.v05() {
            f.field("num_glyphs", &v05.num_glyphs.get());
        }
        if let Some(v1) = self.v1() {
            f.field("max_points", &v1.max_points.get());
            f.field("max_contours", &v1.max_contours.get());
            f.field("max_composite_points", &v1.max_composite_points.get());
            f.field("max_composite_contours", &v1.max_composite_contours.get());
            f.field("max_zones", &v1.max_zones.get());
            f.field("max_twilight_points", &v1.max_twilight_points.get());
            f.field("max_storage", &v1.max_storage.get());
            f.field("max_function_defs", &v1.max_function_defs.get());
            f.field("max_instruction_defs", &v1.max_instruction_defs.get());
            f.field("max_stack_elements", &v1.max_stack_elements.get());
            f.field("max_size_of_instructions", &v1.max_size_of_instructions.get());
            f.field("max_component_elements", &v1.max_component_elements.get());
            f.field("max_component_depth", &v1.max_component_depth.get());
        }

        f.finish()
    }
}

// Monomorphize all fmts to the above fmt for Maxp<'a>
impl std::fmt::Debug for MaxpV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Maxp { maxp: self }.fmt(f)
    }
}
impl std::fmt::Debug for MaxpV05 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Maxp { maxp: self }.fmt(f)
    }
}
impl std::fmt::Debug for MaxpV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Maxp { maxp: self }.fmt(f)
    }
}
