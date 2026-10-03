use crate::util::DisplayBuffer;

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, Default, PartialEq, Eq)]
pub struct ScriptMetrics {
    pub x_size: i16,
    pub y_size: i16,
    pub x_offset: i16,
    pub y_offset: i16,
}

impl ScriptMetrics {
    pub const fn new(x_size: i16, y_size: i16, x_offset: i16, y_offset: i16) -> Self {
        Self { x_size, y_size, x_offset, y_offset }
    }
    pub const fn size(&self) -> (i16, i16) {
        (self.x_size, self.y_size)
    }
    pub const fn offset(&self) -> (i16, i16) {
        (self.x_offset, self.y_offset)
    }
}

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, Default, PartialEq, Eq)]
pub struct LineMetrics {
    pub thickness: i16,
    pub position: i16,
}

impl LineMetrics {
    pub const fn new(thickness: i16, position: i16) -> Self {
        Self { thickness, position }
    }
}

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, Default, PartialEq, Eq)]
pub struct TypographicMetrics {
    pub ascender: i16,  // Note: supposed to be FWORD
    pub descender: i16, // Note: supposed to be FWORD
    pub line_gap: i16,  // Note: supposed to be FWORD
}

impl TypographicMetrics {
    pub const fn new(ascender: i16, descender: i16, line_gap: i16) -> Self {
        Self { ascender, descender, line_gap }
    }
    pub const fn line_spacing(&self) -> i16 {
        self.ascender - self.descender + self.line_gap
    }
}

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, Default, PartialEq, Eq)]
pub struct WindowsMetrics {
    pub ascent: u16,  // Note: supposed to be UFWORD
    pub descent: u16, // Note: supposed to be UFWORD
}

impl WindowsMetrics {
    pub const fn new(ascent: u16, descent: u16) -> Self {
        Self { ascent, descent }
    }
    pub const fn line_spacing(&self) -> u16 {
        self.ascent + self.descent
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct EmbeddingRights: u16 {
        const INSTALLABLE = 0x0000;
        // const RESERVED = 0x0001;
        const RESTRICTED = 0x0002;
        const PREVIEW_AND_PRINT = 0x0004;
        const EDITABLE = 0x0008;
        // const RESERVED = 0x0010;
        // const RESERVED = 0x0020;
        // const RESERVED = 0x0040;
        // const RESERVED = 0x0080;
        const NO_SUBSETTING = 0x0100;
        const BITMAP_ONLY = 0x0200;
        // const RESERVED = 0x0400;
        // const RESERVED = 0x0800;
        // const RESERVED = 0x1000;
        // const RESERVED = 0x2000;
        // const RESERVED = 0x4000;
        // const RESERVED = 0x8000;
    }
}

impl EmbeddingRights {
    pub const fn new_truncating(flags: u16, version: u16) -> Self {
        let mask = match version {
            0..=1 => Self::USAGE_PERMISSIONS_MASK.bits(),
            _ => u16::MAX,
        };
        Self::from_bits_retain(flags & mask)
    }

    const USAGE_PERMISSIONS_MASK: Self =
        Self::RESTRICTED.union(Self::PREVIEW_AND_PRINT).union(Self::EDITABLE);

    // Note: v3 specifies that usage permissions are mutually exclusive, but in v0 to v2
    // we need to have the least restrictive permissions take precedence.

    pub const fn can_embed_bitmaps(&self) -> bool {
        // The only case in which embedding is not allowed is when RESTRICTED is defined alone.
        self.intersection(Self::USAGE_PERMISSIONS_MASK).bits() != Self::RESTRICTED.bits()
    }
    pub const fn can_embed_outlines(&self) -> bool {
        // Outlines can be embedded, if bitmaps can be embedded, and it doesn't have BITMAP_ONLY.
        self.can_embed_bitmaps() && !self.intersects(Self::BITMAP_ONLY)
    }
    pub const fn can_edit(&self) -> bool {
        // The document can be edited if it's INSTALLABLE, or any flags combination with EDITABLE.
        matches!(self.intersection(Self::USAGE_PERMISSIONS_MASK).bits(), 0 | 8..)
    }
    pub const fn can_install(&self) -> bool {
        // The font can be installed on the remote machine only if it's INSTALLABLE.
        !self.intersects(Self::USAGE_PERMISSIONS_MASK)
    }
    pub const fn can_subset(&self) -> bool {
        !self.intersects(Self::NO_SUBSETTING)
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct SelectionFlags: u16 {
        const ITALIC = 0x0001;
        const UNDERSCORE = 0x0002;
        const NEGATIVE = 0x0004;
        const OUTLINED = 0x0008;
        const STRIKEOUT = 0x0010;
        const BOLD = 0x0020;
        const REGULAR = 0x0040;
        const USE_TYPO_METRICS = 0x0080;
        const WWS = 0x0100;
        const OBLIQUE = 0x0200;
        // const RESERVED = 0x0400;
        // const RESERVED = 0x0800;
        // const RESERVED = 0x1000;
        // const RESERVED = 0x2000;
        // const RESERVED = 0x4000;
        // const RESERVED = 0x8000;
    }
}

impl SelectionFlags {
    pub const fn new_truncating(flags: u16, version: u16) -> Self {
        let mask = match version {
            0..=3 => 0x007F,
            4..=5 => 0x03FF,
            _ => u16::MAX,
        };
        Self::from_bits_retain(flags & mask)
    }

    pub const fn is_italic(&self) -> bool {
        self.intersects(Self::ITALIC)
    }
    pub const fn is_underscore(&self) -> bool {
        self.intersects(Self::UNDERSCORE)
    }
    pub const fn is_negative(&self) -> bool {
        self.intersects(Self::NEGATIVE)
    }
    pub const fn is_outlined(&self) -> bool {
        self.intersects(Self::OUTLINED)
    }
    pub const fn is_strikeout(&self) -> bool {
        self.intersects(Self::STRIKEOUT)
    }
    pub const fn is_bold(&self) -> bool {
        self.intersects(Self::BOLD)
    }
    pub const fn is_regular(&self) -> bool {
        self.intersects(Self::REGULAR)
    }
    pub const fn use_typo_metrics(&self) -> bool {
        self.intersects(Self::USE_TYPO_METRICS)
    }
    pub const fn is_wws(&self) -> bool {
        self.intersects(Self::WWS)
    }
    pub const fn is_oblique(&self) -> bool {
        self.intersects(Self::OBLIQUE)
    }
}

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Panose(pub [u8; 10]);

impl Default for Panose {
    fn default() -> Self {
        Self([1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    }
}

impl Panose {
    pub const fn from_bytes(digits: [u8; 10]) -> Self {
        Self(digits)
    }
    pub const fn to_bytes(self) -> [u8; 10] {
        self.0
    }
    pub const fn as_bytes(&self) -> &[u8; 10] {
        &self.0
    }
}

impl std::fmt::Debug for Panose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Max length is 47: "Panose[255 255 255 255 255 255 255 255 255 255]"
        let mut buf = DisplayBuffer::<48>::new();
        buf.write_str_unchecked("Panose[");

        for i in self.0 {
            buf.write_number_u8_unchecked(i);
            buf.write_byte_unchecked(b' ');
        }

        buf.overwrite_last_byte_unchecked(b']');
        f.write_str(buf.as_str())
    }
}
