use crate::{tables::cmap::format8::SequentialMapGroup, types::uint32};

#[repr(C)]
#[non_exhaustive]
pub struct Format12 {
    pub num_groups: uint32,
    groups: [SequentialMapGroup; 0],
}

impl super::CmapSubtableTrait for Format12 {
    const FORMAT: u16 = 12;
}

// TODO: 'cmap' subtable format 12
