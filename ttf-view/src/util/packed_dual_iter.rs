use std::{cmp::Ordering, marker::PhantomData, num::NonZero, ptr::NonNull};

/// A dual iterator, that iterates through either `&[A]` or `&[B]`, using only 2 pointers worth
/// of memory. When iterating through `&[A]`, pointers are stored as `(start, end)`, and when
/// iterating through `&[B]` - as `(end, start)`, removing the need for a extra field.
#[derive(Clone)]
pub struct PackedDualIter<'a, T, A, B> {
    ptr_a: NonNull<()>,
    ptr_b: NonNull<()>,
    _phantom: PhantomData<&'a (T, A, B)>,
}

impl<'a, T: From<A> + From<B>, A: Copy, B: Copy> PackedDualIter<'a, T, A, B> {
    pub const fn new_a(slice: &'a [A]) -> Self {
        let start = NonNull::from_ref(slice).cast::<()>();
        let end = unsafe { start.byte_add(size_of_val(slice)) };
        Self { ptr_a: start, ptr_b: end, _phantom: Default::default() }
    }
    pub const fn new_b(slice: &'a [B]) -> Self {
        let start = NonNull::from_ref(slice).cast::<()>();
        let end = unsafe { start.byte_add(size_of_val(slice)) };
        Self { ptr_a: end, ptr_b: start, _phantom: Default::default() }
    }

    // pub fn as_slice_a(&self) -> Option<&'a [A]> {
    //     if self.ptr_a < self.ptr_b { Some(self._a_slice()) } else { None }
    // }
    // pub fn as_slice_b(&self) -> Option<&'a [B]> {
    //     if self.ptr_b < self.ptr_a { Some(self._b_slice()) } else { None }
    // }

    fn _a_read_start(&self) -> T {
        T::from(unsafe { self.ptr_a.cast::<A>().read() })
    }
    fn _a_read_end(&self) -> T {
        T::from(unsafe { self.ptr_b.cast::<A>().read() })
    }
    fn _b_read_start(&self) -> T {
        T::from(unsafe { self.ptr_b.cast::<B>().read() })
    }
    fn _b_read_end(&self) -> T {
        T::from(unsafe { self.ptr_a.cast::<B>().read() })
    }

    fn _a_advance(&mut self, n: usize) {
        self.ptr_a = unsafe { self.ptr_a.byte_add(n * size_of::<A>()) };
    }
    fn _b_advance(&mut self, n: usize) {
        self.ptr_b = unsafe { self.ptr_b.byte_add(n * size_of::<B>()) };
    }
    fn _a_advance_back(&mut self, n: usize) {
        self.ptr_b = unsafe { self.ptr_b.byte_sub(n * size_of::<A>()) };
    }
    fn _b_advance_back(&mut self, n: usize) {
        self.ptr_a = unsafe { self.ptr_a.byte_sub(n * size_of::<B>()) };
    }

    fn _a_len(&self) -> usize {
        unsafe { self.ptr_b.byte_offset_from_unsigned(self.ptr_a) / size_of::<A>() }
    }
    fn _b_len(&self) -> usize {
        unsafe { self.ptr_a.byte_offset_from_unsigned(self.ptr_b) / size_of::<B>() }
    }
    fn _a_slice(&self) -> &'a [A] {
        unsafe { NonNull::slice_from_raw_parts(self.ptr_a.cast::<A>(), self._a_len()).as_ref() }
    }
    fn _b_slice(&self) -> &'a [B] {
        unsafe { NonNull::slice_from_raw_parts(self.ptr_b.cast::<B>(), self._b_len()).as_ref() }
    }
}

impl<'a, T: From<A> + From<B>, A: Copy, B: Copy> Iterator for PackedDualIter<'a, T, A, B> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr_a.cmp(&self.ptr_b) {
            Ordering::Equal => None,
            Ordering::Less => {
                let item = self._a_read_start();
                self._a_advance(1);
                Some(item)
            },
            Ordering::Greater => {
                let item = self._b_read_start();
                self._b_advance(1);
                Some(item)
            },
        }
    }
    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        match self.ptr_a.cmp(&self.ptr_b) {
            Ordering::Equal => NonZero::new(n).map_or(Ok(()), Err),
            Ordering::Less => {
                let advance = self._a_len().min(n);
                self._a_advance(advance);
                NonZero::new(n - advance).map_or(Ok(()), Err)
            },
            Ordering::Greater => {
                let advance = self._b_len().min(n);
                self._b_advance(advance);
                NonZero::new(n - advance).map_or(Ok(()), Err)
            },
        }
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.advance_by(n).ok()?;
        self.next()
    }
    fn try_fold<I, F, R>(&mut self, init: I, f: F) -> R
    where
        F: FnMut(I, Self::Item) -> R,
        R: std::ops::Try<Output = I>,
    {
        match self.ptr_a.cmp(&self.ptr_b) {
            Ordering::Equal => R::from_output(init),
            Ordering::Less => self._a_slice().iter().copied().map(T::from).try_fold(init, f),
            Ordering::Greater => self._b_slice().iter().copied().map(T::from).try_fold(init, f),
        }
    }
    fn fold<I, F>(mut self, init: I, mut f: F) -> I
    where F: FnMut(I, Self::Item) -> I {
        self.try_fold(init, |init, x| Ok::<_, !>(f(init, x))).unwrap()
    }
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T: From<A> + From<B>, A: Copy, B: Copy> ExactSizeIterator for PackedDualIter<'a, T, A, B> {
    fn is_empty(&self) -> bool {
        self.ptr_a == self.ptr_b
    }
    fn len(&self) -> usize {
        if self.ptr_a < self.ptr_b { self._a_len() } else { self._b_len() }
    }
}

impl<'a, T: From<A> + From<B>, A: Copy, B: Copy> DoubleEndedIterator
    for PackedDualIter<'a, T, A, B>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.ptr_a.cmp(&self.ptr_b) {
            Ordering::Equal => None,
            Ordering::Less => {
                self._a_advance_back(1);
                Some(self._a_read_end())
            },
            Ordering::Greater => {
                self._b_advance_back(1);
                Some(self._b_read_end())
            },
        }
    }
    fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        match self.ptr_a.cmp(&self.ptr_b) {
            Ordering::Equal => NonZero::new(n).map_or(Ok(()), Err),
            Ordering::Less => {
                let advance = self._a_len().min(n);
                self._a_advance_back(advance);
                NonZero::new(n - advance).map_or(Ok(()), Err)
            },
            Ordering::Greater => {
                let advance = self._b_len().min(n);
                self._b_advance_back(advance);
                NonZero::new(n - advance).map_or(Ok(()), Err)
            },
        }
    }
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.advance_back_by(n).ok()?;
        self.next_back()
    }
    fn try_rfold<I, F, R>(&mut self, init: I, f: F) -> R
    where
        F: FnMut(I, Self::Item) -> R,
        R: std::ops::Try<Output = I>,
    {
        match self.ptr_a.cmp(&self.ptr_b) {
            Ordering::Equal => R::from_output(init),
            Ordering::Less => self._a_slice().iter().copied().map(T::from).try_rfold(init, f),
            Ordering::Greater => self._b_slice().iter().copied().map(T::from).try_rfold(init, f),
        }
    }
    fn rfold<I, F>(mut self, init: I, mut f: F) -> I
    where F: FnMut(I, Self::Item) -> I {
        self.try_rfold(init, |init, x| Ok::<_, !>(f(init, x))).unwrap()
    }
}
