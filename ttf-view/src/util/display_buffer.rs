use std::mem::MaybeUninit;

/// DisplayBuffer allows things to be formatted on the stack, quickly and in inline-friendly way,
/// without spamming `&dyn Write` with calls to write a single character or a tiny fragment. After
/// the thing is formatted on-stack, it can then finally be written to `&dyn Write` with one call.
pub(crate) struct DisplayBuffer<const N: usize> {
    pos: usize,
    buf: [MaybeUninit<u8>; N],
}

impl<const N: usize> DisplayBuffer<N> {
    pub const fn new() -> Self {
        Self { pos: 0, buf: [MaybeUninit::uninit(); N] }
    }

    #[inline]
    pub fn write_str_unchecked(&mut self, s: &str) {
        debug_assert!(self.pos + s.len() <= N);

        let dst = unsafe { self.buf.get_unchecked_mut(self.pos..self.pos + s.len()) };
        dst.write_copy_of_slice(s.as_bytes());
        self.pos += s.len();
    }
    #[inline]
    pub fn write_byte_unchecked(&mut self, byte: u8) {
        debug_assert!(self.pos < N && byte.is_ascii());

        unsafe { self.buf.get_unchecked_mut(self.pos) }.write(byte);
        self.pos += 1;
    }
    #[inline]
    pub fn write_two_digits_unchecked(&mut self, num: u8) {
        debug_assert!(self.pos + 2 <= N && num < 100);

        let dst = unsafe { self.buf.get_unchecked_mut(self.pos..self.pos + 2) };
        dst.write_copy_of_slice(&[b'0' + num / 10, b'0' + num % 10]);
        self.pos += 2;
    }

    pub const fn as_str(&self) -> &str {
        debug_assert!(self.pos <= N);
        unsafe { str::from_utf8_unchecked(self.buf.get_unchecked(..self.pos).assume_init_ref()) }
    }
}

impl<const N: usize> std::fmt::Write for DisplayBuffer<N> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf
            .get_mut(self.pos..(self.pos + s.len()))
            .map(|dst| {
                dst.write_copy_of_slice(s.as_bytes());
                self.pos += s.len();
            })
            .ok_or(std::fmt::Error)
    }
}
