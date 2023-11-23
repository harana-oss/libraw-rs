use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::slice;

use libraw_sys::{libraw_dcraw_clear_mem, libraw_processed_image_t};

use crate::BitDepth;

pub struct Image<T> {
    inner: *mut libraw_processed_image_t,
    marker_: PhantomData<T>,
}

impl<T: BitDepth> Image<T> {
    pub(crate) unsafe fn from_raw(ptr: *mut libraw_processed_image_t) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert_eq!((*ptr).bits as usize, mem::size_of::<T>() * 8);

        Self {
            inner: ptr,
            marker_: PhantomData,
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { (*self.inner).width }.into()
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.inner).height }.into()
    }
}

impl Deref for Image<u8> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(
                (*self.inner).data.as_ptr(),
                (*self.inner).data_size as usize,
            )
        }
    }
}

impl Deref for Image<u16> {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        unsafe {
            debug_assert_eq!((*self.inner).data.as_ptr() as usize % 2, 0);

            slice::from_raw_parts(
                (*self.inner).data.as_ptr() as *const u16,
                (*self.inner).data_size as usize / 2,
            )
        }
    }
}

impl<T> Drop for Image<T> {
    fn drop(&mut self) {
        unsafe { libraw_dcraw_clear_mem(self.inner) }
    }
}
