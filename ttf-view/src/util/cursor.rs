use std::ptr::NonNull;

#[derive_const(Clone)]
pub struct Checked(*const ());
#[derive_const(Clone)]
pub struct Unchecked;

pub trait Check: const Clone {
    type Err;
    fn check_in_range<T>(&self, ptr: NonNull<T>) -> Result<(), Self::Err>;
    fn check_not_past_end<T>(&self, ptr: NonNull<T>) -> Result<(), Self::Err>;
}

impl Check for Checked {
    type Err = ();
    fn check_in_range<T>(&self, ptr: NonNull<T>) -> Result<(), Self::Err> {
        if ptr.cast::<()>().as_ptr().cast_const() < self.0 { Ok(()) } else { Err(()) }
    }
    fn check_not_past_end<T>(&self, ptr: NonNull<T>) -> Result<(), Self::Err> {
        if ptr.cast::<()>().as_ptr().cast_const() <= self.0 { Ok(()) } else { Err(()) }
    }
}
impl Check for Unchecked {
    type Err = !;
    fn check_in_range<T>(&self, _ptr: NonNull<T>) -> Result<(), Self::Err> {
        Ok(())
    }
    fn check_not_past_end<T>(&self, _ptr: NonNull<T>) -> Result<(), Self::Err> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct Cursor<'a, T, C> {
    ptr: NonNull<T>,
    check: C,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Cursor<'a, T, Checked> {
    pub const fn new_in(slice: &'a [T]) -> Self {
        Self::new_at_with(NonNull::from_ref(slice).cast(), Checked(slice.as_ptr_range().end.cast()))
    }
}
impl<'a, T> Cursor<'a, T, Unchecked> {
    pub const fn new_at(ptr: NonNull<T>) -> Self {
        Self::new_at_with(ptr, Unchecked)
    }
}

impl<'a, T, C: Check> Cursor<'a, T, C> {
    pub const fn new_at_with(ptr: NonNull<T>, check: C) -> Self {
        Self { ptr, check, _phantom: Default::default() }
    }

    pub fn peek_as<U>(&self) -> Result<&'a U, C::Err> {
        const { assert!(size_of::<U>().is_multiple_of(size_of::<T>())) };
        self.check.check_in_range(self.ptr)?;
        Ok(unsafe { self.ptr.cast::<U>().as_ref() })
    }
    pub fn advance_by_bytes(&mut self, byte_offset: usize) -> Result<(), C::Err> {
        let new_ptr = unsafe { self.ptr.byte_add(byte_offset) };
        self.check.check_not_past_end(new_ptr)?;
        self.ptr = new_ptr;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, C::Err>
    where T: Copy {
        self.read_as::<T>()
    }
    pub fn read_as<U: Copy>(&mut self) -> Result<U, C::Err> {
        let value = *self.peek_as::<U>()?;
        self.ptr = unsafe { self.ptr.byte_add(size_of::<U>()) };
        Ok(value)
    }
    // pub fn read_ref(&mut self) -> Result<&'a T, C::Err> {
    //     self.read_ref_as::<T>()
    // }
    pub fn read_ref_as<U>(&mut self) -> Result<&'a U, C::Err> {
        let value = self.peek_as::<U>()?;
        self.ptr = unsafe { self.ptr.byte_add(size_of::<U>()) };
        Ok(value)
    }

    pub const fn cast<U>(&self) -> Cursor<'a, U, C> {
        Cursor::<'a, U, C>::new_at_with(self.ptr.cast::<U>(), self.check.clone())
    }
    pub const fn offset_cast<U>(&self, byte_offset: usize) -> Cursor<'a, U, C> {
        let ptr = unsafe { self.ptr.byte_add(byte_offset) }.cast::<U>();
        Cursor::<'a, U, C>::new_at_with(ptr, self.check.clone())
    }

    pub const fn offset_from(&self, other: NonNull<T>) -> usize {
        unsafe { self.ptr.offset_from_unsigned(other) }
    }
    pub const fn as_non_null(&self) -> NonNull<T> {
        self.ptr
    }
    // pub const fn as_ptr(&self) -> *const T {
    //     self.ptr.as_ptr()
    // }
}
