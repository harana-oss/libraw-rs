use std::convert::TryFrom;
use std::ops::Deref;
use std::slice;

use libraw_sys::*;

#[derive(Eq, PartialEq, Debug)]
pub enum ThumbnailFormat {
    Unknown,
    Jpeg,
    Bitmap,
    Bitmap16,
    Layer,
    Rollei
}

impl From<LibRaw_thumbnail_formats> for ThumbnailFormat {
    fn from(t: LibRaw_thumbnail_formats) -> Self {
        match t {
            LIBRAW_THUMBNAIL_UNKNOWN    => ThumbnailFormat::Unknown,
            LIBRAW_THUMBNAIL_JPEG       => ThumbnailFormat::Jpeg,
            LIBRAW_THUMBNAIL_BITMAP     => ThumbnailFormat::Bitmap,
            LIBRAW_THUMBNAIL_BITMAP16   => ThumbnailFormat::Bitmap16,
            LIBRAW_THUMBNAIL_LAYER      => ThumbnailFormat::Layer,
            LIBRAW_THUMBNAIL_ROLLEI     => ThumbnailFormat::Rollei,
            _                           => unreachable!("LibRaw_thumbnail_formats has been expanded; report this to libraw-rs"),
        }
    }
}

pub struct ImagePreview<'a> {
    inner: &'a libraw_thumbnail_t
}

impl<'a> ImagePreview<'a> {
    pub(crate) unsafe fn from_raw(inner: &'a libraw_thumbnail_t) -> Self {
        Self {
            inner
        }
    }

    pub fn format(&self) -> ThumbnailFormat {
        ThumbnailFormat::from(self.inner.tformat)
    }
}

impl<'a> Deref for ImagePreview<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(
                self.inner.thumb as *mut u8,
                usize::try_from(self.inner.tlength).unwrap()
            )
        }
    }
}
