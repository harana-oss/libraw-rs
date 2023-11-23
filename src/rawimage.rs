use std::ops::Deref;
use std::slice;

use crate::{Reader, Sizes};

pub struct RawImage {
    reader: Reader,
}

impl RawImage {
    pub(crate) fn new(reader: Reader) -> Self {
        debug_assert!(!unsafe { (*reader.inner).rawdata.raw_alloc }.is_null());

        Self { reader }
    }

    pub fn sizes(&self) -> Sizes {
        Sizes::new(unsafe { (*self.reader.inner).sizes })
    }
}

impl Deref for RawImage {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        let sizes = self.sizes();

        unsafe {
            slice::from_raw_parts(
                (*self.reader.inner).rawdata.raw_image,
                sizes.raw_width as usize * sizes.raw_height as usize,
            )
        }
    }
}
