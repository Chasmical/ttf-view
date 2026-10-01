use crate::{
    tables::glyf::Glyph,
    types::{int16, uint16},
    util::{Check, Cursor, Unchecked},
};
use std::ptr::NonNull;

#[repr(C)]
pub struct SimpleGlyph {
    base: Glyph,
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
        &self.base
    }
}

impl SimpleGlyph {
    pub const fn end_pts_of_contours(&self) -> &[uint16] {
        let len = self.number_of_contours.get() as usize;
        unsafe { std::slice::from_raw_parts(self.end_pts_of_contours.as_ptr(), len) }
    }
    pub const fn total_point_count(&self) -> u32 {
        // TODO: When closures are constified, replace this with Option::map_or
        match self.end_pts_of_contours().last() {
            Some(last_idx) => last_idx.get() as u32 + 1,
            None => 0,
        }
    }
    pub const fn instruction_length(&self) -> uint16 {
        unsafe { *self.end_pts_of_contours().as_ptr_range().end }
    }
    pub const fn instructions(&self) -> &[u8] {
        let len_ptr = self.end_pts_of_contours().as_ptr_range().end;
        let len = unsafe { &*len_ptr }.get() as usize;
        unsafe { std::slice::from_raw_parts(len_ptr.byte_add(2).cast(), len) }
    }
    pub const fn flags_ptr(&self) -> NonNull<PointFlags> {
        let ptr = self.instructions().as_ptr_range().end;
        NonNull::from_ref(unsafe { &*ptr.cast() })
    }

    pub fn contours(&self) -> ContoursIter<'_> {
        ContoursIter::new(self)
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct PointFlags: u8 {
        const ON_CURVE_POINT = 0x01;
        const X_IS_SHORT = 0x02;
        const Y_IS_SHORT = 0x04;
        const REPEAT_FLAG = 0x08;
        const X_IS_SAME_OR_POS = 0x10;
        const Y_IS_SAME_OR_POS = 0x20;
        const OVERLAP_SIMPLE = 0x40;
        // const RESERVED = 0x80;
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
    pub const fn is_repeated(&self) -> bool {
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

impl Point {
    pub const fn on_curve(&self) -> bool {
        self.flags.on_curve()
    }
}

/// This function calculates the lengths of variable-length arrays (flags, x_coords, y_coords),
/// reading up to specified number of logical points. Here's some cases to better understand this:
///
/// ```text
/// Flags: (short_x | long_y)
/// - Read 1 flags byte, a short (1-byte) X coord, and a long (2-byte) Y coord. 1 logical point.
///
/// Flags: (repeat | long_x | same_y) (5)
///   Read 2 flags bytes, 5 long X coords, and NO Y coords (they're zero). 5 logical points.
///
/// Flags: (repeat | short_x | short_y) (20)
/// - Read 2 flags bytes, 20 short X coords and 20 short Y coords. 20 logical points.
/// ```
///
/// TODO: Passing old state and returning new state is a bit of an antipattern...
/// I should maybe probably move this to a method on ContoursIter?
pub(super) fn process_contour_data<C: Check>(
    mut cursor: Cursor<'_, PointFlags, C>,
    mut points_to_read: u32,
    mut xs_len: u32,
    mut ys_len: u32,
) -> Result<(u32, u32, u32, PointFlags, u8), C::Err> {
    let started_from = cursor.as_non_null();

    // Number of points can't exceed 65536 (65535 is max possible index in end_pts_of_contours),
    // and maximum value of xs_len/ys_len is twice that (131072) and need to be stored in u32.
    let mut flags = PointFlags::empty();
    let mut extra_count = 0;

    // Read flags with cursor until we've read all the logical points we need
    while points_to_read > 0 {
        flags = cursor.read()?;

        // Calculate the amount of logical flags (just one, or repeated N times)
        let repeated_count = if flags.is_repeated() { 1 + cursor.read()?.bits() as u32 } else { 1 };

        // If a repeated flags byte crosses to the next contour, we'll need to store and return the
        // amount of extra repeats of these flags, that should be carried over to the next contour.
        let actual_count = repeated_count.min(points_to_read);
        if repeated_count > actual_count {
            extra_count = (repeated_count - actual_count) as u8;
        }
        points_to_read -= actual_count;

        // Add how many bytes the X and Y coordinates with these flags would occupy
        xs_len += calc_xs_len(flags, actual_count);
        ys_len += calc_ys_len(flags, actual_count);
    }

    // Calculate how many flags were read
    let flags_len = cursor.offset_from(started_from) as u32;

    Ok((flags_len, xs_len, ys_len, flags, extra_count))
}

fn calc_xs_len(flags: PointFlags, actual_count: u32) -> u32 {
    // Legend: f=flags, A=f&2 (short), B=f&16 (same_or_pos), C=f&18 (both).
    //
    // | X type   | | A |A=0|A≠0| |B=0|B≠0| |C=0|C≠0|C=2|C≠2|C=16|C≠16|C=18|C≠18| |sum|
    // |----------| |---|---|---| |---|---| |---|---|---|---|----|----|----|----| |---|
    // | long   0 | | 0 | 1 | 0 | | 1 | 0 | | 1 | 0 | 0 | 1 | 0  | 1  | 0  | 1  | | 2 |
    // | -byte  2 | | 0 | 1 | 0 | | 0 | 1 | | 0 | 1 | 1 | 0 | 0  | 1  | 0  | 1  | | 1 |
    // | zero  16 | | 2 | 0 | 1 | | 1 | 0 | | 0 | 1 | 0 | 1 | 1  | 0  | 0  | 1  | | 0 |
    // | +byte 18 | | 2 | 0 | 1 | | 0 | 1 | | 0 | 1 | 0 | 1 | 0  | 1  | 1  | 0  | | 1 |
    //                                        ^                    ^
    // (C=0)+(C≠16) appears to be the simplest solution.
    // This solution works for Y too (just different bitmasks).

    let c = flags.intersection(PointFlags::X_IS_SHORT.union(PointFlags::X_IS_SAME_OR_POS));
    (c.is_empty() as u32 + (c != PointFlags::X_IS_SAME_OR_POS) as u32) * actual_count
}
fn calc_ys_len(flags: PointFlags, actual_count: u32) -> u32 {
    let c = flags.intersection(PointFlags::Y_IS_SHORT.union(PointFlags::Y_IS_SAME_OR_POS));
    (c.is_empty() as u32 + (c != PointFlags::Y_IS_SAME_OR_POS) as u32) * actual_count
}

#[derive(Clone)]
pub struct ContoursIter<'a> {
    end_points: std::slice::Iter<'a, uint16>,
    flags: Cursor<'a, PointFlags, Unchecked>,
    x_coords: Cursor<'a, u8, Unchecked>,
    y_coords: Cursor<'a, u8, Unchecked>,
    prev_end_point: u16,
    prev_repeat_flags: PointFlags,
    prev_repeat_count: u8,
}

impl<'a> ContoursIter<'a> {
    pub fn new(glyph: &'a SimpleGlyph) -> Self {
        let flags = Cursor::new_at(glyph.flags_ptr());

        let end_points = glyph.end_pts_of_contours();
        let points_count = glyph.total_point_count();

        // We'll use Cursor<'_, _, Unchecked> to read the flags array, since the SimpleGlyph
        // is guaranteed to be well-formed, thanks to Glyf::new_in's validation.
        let data = process_contour_data(flags.clone(), points_count, 0, 0).unwrap();
        let (flags_len, xs_len, _ys_len, _, extra_count) = data;

        // We just calculated the sizes of the arrays, so we don't need the extra repeat flags,
        // but it definitely should be 0 if everything works as it should (just a sanity check).
        debug_assert!(extra_count == 0);

        // Create Cursor<'_, _, Unchecked>s for the coordinates
        let x_coords = flags.offset_cast(flags_len as usize);
        let y_coords = x_coords.offset_cast(xs_len as usize);

        Self {
            end_points: end_points.iter(),
            flags,
            x_coords,
            y_coords,
            prev_end_point: 0,
            prev_repeat_flags: PointFlags::empty(),
            prev_repeat_count: 0,
        }
    }
}

impl<'a> Iterator for ContoursIter<'a> {
    type Item = Contour<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        // Read the next point, and calculate the amount of points in this contour
        let end_point = self.end_points.next()?.get();
        let points_count = (end_point - self.prev_end_point) as u32 + 1;
        self.prev_end_point = end_point;

        // Check how many flags bytes are still "in the buffer" from the last repeated flags,
        // and calculate the lengths of the corresponding X and Y coordinates.
        let remained_from_previous = (self.prev_repeat_count as u32).min(points_count);
        let xs_len = calc_xs_len(self.prev_repeat_flags, remained_from_previous);
        let ys_len = calc_ys_len(self.prev_repeat_flags, remained_from_previous);
        self.prev_repeat_count -= remained_from_previous as u8;

        // Read however many logical points we need to "complete" this contour. Could be zero,
        // meaning that we shouldn't override the prev_repeat_flags/prev_repeat_count with the
        // values we get here just yet - those repeated flags are still not exhausted.
        let to_read = points_count - remained_from_previous;
        let data = process_contour_data(self.flags.clone(), to_read, xs_len, ys_len).unwrap();
        let (flags_len, xs_len, ys_len, repeat_flags, repeat_count) = data;

        let contour = Contour {
            flags: self.flags.clone(),
            x_coords: self.x_coords.clone(),
            y_coords: self.y_coords.clone(),
            points_left: points_count,
            // Contour will return `remained_from_previous` points with the last repeated flags,
            // before moving on to the points with flags dedicated to this current contour.
            repeat_count: remained_from_previous as u8,
            cur_flags: self.prev_repeat_flags,
        };

        // If the repeat_count we got from process_contour_data is not zero, that means we actually
        // got to read some flags, meaning that the last repeated flags were exhausted. So we need
        // to replace the old repeated flags with the new ones.
        if repeat_count > 0 {
            self.prev_repeat_flags = repeat_flags;
            self.prev_repeat_count = repeat_count;
        }

        // Advance to the starting points of the next contour
        self.flags.advance_by_bytes(flags_len as usize);
        self.x_coords.advance_by_bytes(xs_len as usize);
        self.y_coords.advance_by_bytes(ys_len as usize);

        // Return the contour we've just read
        Some(contour)
    }
}
impl<'a> ExactSizeIterator for ContoursIter<'a> {
    fn len(&self) -> usize {
        self.end_points.len()
    }
}
impl<'a> std::iter::FusedIterator for ContoursIter<'a> {}

#[derive(Clone)]
pub struct Contour<'a> {
    flags: Cursor<'a, PointFlags, Unchecked>,
    x_coords: Cursor<'a, u8, Unchecked>,
    y_coords: Cursor<'a, u8, Unchecked>,
    points_left: u32,
    cur_flags: PointFlags,
    repeat_count: u8,
}

impl<'a> Iterator for Contour<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        // We need to keep track of how many points are left, so that we don't accidentally
        // return the next contour's points with the repeated flags at the end of this one.
        self.points_left = self.points_left.checked_sub(1)?;

        // If we're currently repeating flags, decrement the counter and use the repeated value
        if self.repeat_count > 0 {
            self.repeat_count -= 1;
        } else {
            // Otherwise, read the next flags, and set the repeat counter if flags are repeated
            self.cur_flags = self.flags.read().unwrap();
            if self.cur_flags.is_repeated() {
                self.repeat_count = self.flags.read().unwrap().bits();
            }
        }

        // TODO: Will this really be better than just some conditions?

        let dx = match calc_xs_len(self.cur_flags, 1) {
            0 => 0,
            1 => {
                let byte = self.x_coords.read().unwrap() as i16;
                if self.cur_flags.is_x_same_or_pos() { byte } else { -byte }
            },
            2 => self.x_coords.read_as::<int16>().unwrap().get(),
            // Hopefully Rust realizes that the sum of two bools can't be more than 2
            _ => unreachable!(),
        };

        let dy = match calc_ys_len(self.cur_flags, 1) {
            0 => 0,
            1 => {
                let byte = self.y_coords.read().unwrap() as i16;
                if self.cur_flags.is_y_same_or_pos() { byte } else { -byte }
            },
            2 => self.y_coords.read_as::<int16>().unwrap().get(),
            // Hopefully Rust realizes that the sum of two bools can't be more than 2
            _ => unreachable!(),
        };

        Some(Point { flags: self.cur_flags, dx, dy })
    }
}
impl<'a> ExactSizeIterator for Contour<'a> {
    fn len(&self) -> usize {
        self.points_left as usize
    }
}
impl<'a> std::iter::FusedIterator for Contour<'a> {}
