use crate::{tables::glyf::Glyph, types::uint16};
use std::ptr::NonNull;

#[repr(C)]
pub struct SimpleGlyph {
    header: Glyph,
    end_pts_of_contours: [uint16; 0],
    // : instruction_length: uint16,
    // : instructions: [u8; instruction_length],
    // : flags: [u8; variable],
    // : x_coordinates: [u8/i16; variable],
    // : y_coordinates: [u8/i16; variable],
}

const impl std::ops::Deref for SimpleGlyph {
    type Target = Glyph;
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl SimpleGlyph {
    pub const fn end_pts_of_contours(&self) -> &[uint16] {
        let len = self.number_of_contours.get() as usize;
        unsafe { std::slice::from_raw_parts(self.end_pts_of_contours.as_ptr(), len) }
    }
    pub const fn instruction_length(&self) -> uint16 {
        unsafe { *self.end_pts_of_contours().as_ptr_range().end }
    }
    pub const fn instructions(&self) -> &[u8] {
        let len_ptr = self.end_pts_of_contours().as_ptr_range().end;
        let len = unsafe { *len_ptr }.get() as usize;
        unsafe { std::slice::from_raw_parts(len_ptr.byte_add(2).cast(), len) }
    }
    pub const fn flags_ptr(&self) -> NonNull<PointFlags> {
        let ptr = self.instructions().as_ptr_range().end.cast_mut();
        unsafe { NonNull::new_unchecked(ptr.cast()) }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PointFlags: u8 {
        const ON_CURVE_POINT = 0x01;
        const X_IS_SHORT = 0x02;
        const Y_IS_SHORT = 0x04;
        const REPEAT_FLAG = 0x08;
        const X_IS_SAME_OR_POS = 0x10;
        const Y_IS_SAME_OR_POS = 0x20;
        const OVERLAP_SIMPLE = 0x40;
        const RESERVED = 0x80;
    }
}

impl PointFlags {
    pub const fn on_curve(&self) -> bool {
        self.intersects(Self::ON_CURVE_POINT)
    }
    pub const fn is_x_short(&self) -> bool {
        self.intersects(Self::X_IS_SHORT)
    }
    pub const fn is_y_short(&self) -> bool {
        self.intersects(Self::Y_IS_SHORT)
    }
    pub const fn is_repeating(&self) -> bool {
        self.intersects(Self::REPEAT_FLAG)
    }
    pub const fn is_x_same_or_pos(&self) -> bool {
        self.intersects(Self::X_IS_SAME_OR_POS)
    }
    pub const fn is_y_same_or_pos(&self) -> bool {
        self.intersects(Self::Y_IS_SAME_OR_POS)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub flags: PointFlags,
    pub dx: i16,
    pub dy: i16,
}

/// PointsIter<'a> will iterate through all flags, and parse the points.
///
/// Flags need to be read until we get past (last end point index + 1)

pub struct PointsIter<'a> {
    end_points_of_contours: NonNull<uint16>,
    number_of_contours: u16,
    repeat_flag: u8,
    flags_ptr: NonNull<PointFlags>,
    x_coords: NonNull<()>,
    y_coords: NonNull<()>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

fn find_lengths(mut ptr: NonNull<PointFlags>, mut points_left: u32) -> (u32, u32, u32) {
    let started_from = ptr;
    let mut xs_len = 0;
    let mut ys_len = 0;

    while points_left > 0 {
        let flag = unsafe { ptr.read() };
        unsafe { ptr = ptr.byte_add(1) };

        let count = if flag.is_repeating() {
            unsafe {
                let repeats = ptr.read().bits();
                ptr = ptr.byte_add(1);
                1 + repeats as u32
            }
        } else {
            1
        };

        points_left = points_left.saturating_sub(count);

        // | short | same_or_pos | value | size |
        // |-------|-------------|-------|------|
        // | 0 | 0 | int16 | 2 |
        // | 0 | 1 | 0     | 0 |
        // | 1 | 0 | - u8  | 1 |
        // | 1 | 1 | + u8  | 1 |

        xs_len += flag.is_x_short() as u32 * count;
        const X_FLAGS: PointFlags = PointFlags::X_IS_SHORT.union(PointFlags::X_IS_SAME_OR_POS);
        xs_len += !flag.intersects(X_FLAGS) as u32 * count * 2;

        ys_len += flag.is_y_short() as u32 * count;
        const Y_FLAGS: PointFlags = PointFlags::Y_IS_SHORT.union(PointFlags::Y_IS_SAME_OR_POS);
        ys_len += !flag.intersects(Y_FLAGS) as u32 * count * 2;
    }

    let flags_len = unsafe { ptr.offset_from_unsigned(started_from) } as u32;
    (flags_len, xs_len, ys_len)
}

impl<'a> PointsIter<'a> {
    pub fn new(glyph: &'a SimpleGlyph) -> Self {
        let flags_ptr = glyph.flags_ptr();
        let points_count = glyph.end_pts_of_contours().last().map_or(0, |x| x.get() as u32) + 1;
        let (flags_len, xs_len, ys_len) = find_lengths(flags_ptr, points_count);

        Self {
            end_points_of_contours: NonNull::from_ref(&glyph.end_pts_of_contours).cast(),
            number_of_contours: glyph.number_of_contours.get() as u16,
            repeat_flag: 0,
            flags_ptr,
            x_coords: unsafe { flags_ptr.byte_add(flags_len as _).cast() },
            y_coords: unsafe { flags_ptr.byte_add((flags_len + xs_len) as _).cast() },
            _phantom: Default::default(),
        }
    }
}

impl<'a> Iterator for PointsIter<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let flag = {
            if self.repeat_flag != 0 {
                self.repeat_flag -= 1;
            } else {
            }
            unsafe { self.flags_ptr.read() }
        };

        todo!()
    }
}
