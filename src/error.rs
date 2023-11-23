use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use libraw_sys::LIBRAW_SUCCESS;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
    code: i32,
}

impl Error {
    pub(crate) fn check(code: i32) -> Result<()> {
        if code == LIBRAW_SUCCESS {
            Ok(())
        } else {
            Err(Error { code })
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "libraw error: {}", self.code)
    }
}

impl StdError for Error {}
