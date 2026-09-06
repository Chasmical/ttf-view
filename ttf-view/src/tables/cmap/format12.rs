use crate::{tables::cmap::format8::SequentialMapGroupRepr, types::uint32};

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat12Repr {
    pub num_groups: uint32,
    groups: [SequentialMapGroupRepr; 0],
}

impl super::CmapSubtableTrait for CmapSubtableFormat12Repr {
    const FORMAT: u16 = 12;
}

// TODO: 'cmap' subtable format 12
