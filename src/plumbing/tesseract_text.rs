extern crate tesseract_sys;

use self::tesseract_sys::TessDeleteText;
use std::convert::AsRef;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Wrapper around Tesseract's returned strings
pub struct TesseractText(*const c_char);

impl Drop for TesseractText {
    fn drop(&mut self) {
        unsafe { TessDeleteText(self.0) }
    }
}

impl TesseractText {
    /// # Safety
    ///
    /// This function should only be called with a valid string pointer from Tesseract.
    /// `TesseractText` will be responsible for freeing it.
    pub unsafe fn new(raw: *const c_char) -> Self {
        Self(raw)
    }
}

impl AsRef<CStr> for TesseractText {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}
