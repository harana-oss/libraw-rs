use std::mem;

use libraw_sys::{libraw_close, libraw_data_t, libraw_dcraw_make_mem_image, libraw_dcraw_process, libraw_init, libraw_open_buffer, libraw_unpack, libraw_unpack_thumb};

use crate::{BitDepth, Error, Image, RawImage, Result};
use crate::thumbnail::ImagePreview;

pub struct Reader {
    pub(crate) inner: *mut libraw_data_t,
}

impl Reader {
    pub fn new() -> Self {
        let inner = unsafe { libraw_init(0) };
        Self { inner }
    }

    fn open(&self, buf: &[u8]) -> Result<()> {
        Error::check(unsafe {
            libraw_open_buffer(self.inner, buf.as_ptr() as *const _, buf.len())
        })?;
        Error::check(unsafe { libraw_unpack(self.inner) })?;

        Ok(())
    }

    pub fn thumbnail(&self, buf: &[u8]) -> Result<ImagePreview> {
        Error::check(unsafe {
            libraw_open_buffer(self.inner, buf.as_ptr() as *const _, buf.len())
        })?;
        Error::check(unsafe { libraw_unpack_thumb(self.inner) })?;

        Ok(unsafe { ImagePreview::from_raw(&(*self.inner).thumbnail)})
    }

    pub fn decode(self, buf: &[u8]) -> Result<RawImage> {
        self.open(buf)?;

        let decoded = RawImage::new(self);
        Ok(decoded)
    }

    #[inline]
    pub fn read_8bit(self, buf: &[u8]) -> Result<Image<u8>> {
        self.read(buf)
    }

    #[inline]
    pub fn read_16bit(self, buf: &[u8]) -> Result<Image<u16>> {
        self.read(buf)
    }

    fn read<T: BitDepth>(self, buf: &[u8]) -> Result<Image<T>> {
        let bps = mem::size_of::<T>() * 8;
        debug_assert!(bps == 8 || bps == 16);
        unsafe { (*self.inner).params.output_bps = bps as i32 };

        self.open(buf)?;
        Error::check(unsafe { libraw_dcraw_process(self.inner) })?;

        let mut result = 0i32;
        let processed = unsafe { libraw_dcraw_make_mem_image(self.inner, &mut result) };
        Error::check(result)?;

        let image = unsafe { Image::from_raw(processed) };
        Ok(image)
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe { libraw_close(self.inner) }
    }
}

impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}
