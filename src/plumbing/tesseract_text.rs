extern crate tesseract_sys;

use self::tesseract_sys::TessDeleteText;
use std::convert::AsRef;
use std::ffi::CStr;
use std::os::raw::c_char;

pub use self::tesseract_sys::{
    TessOcrEngineMode, TessOcrEngineMode_OEM_DEFAULT, TessOcrEngineMode_OEM_LSTM_ONLY,
    TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED, TessOcrEngineMode_OEM_TESSERACT_ONLY,
};

/// Wrapper around Tesseract's returned strings
pub struct TesseractText(*mut c_char);

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
    pub unsafe fn new(raw: *mut c_char) -> Self {
        Self(raw)
    }
}

impl AsRef<CStr> for TesseractText {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}
