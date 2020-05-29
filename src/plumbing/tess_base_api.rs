extern crate tesseract_sys;
extern crate thiserror;

use self::tesseract_sys::{
    TessBaseAPICreate, TessBaseAPIDelete, TessBaseAPIGetUTF8Text, TessBaseAPIInit3,
    TessBaseAPIRecognize, TessBaseAPISetImage, TessBaseAPISetImage2,
    TessBaseAPISetSourceResolution, TessBaseAPISetVariable,
};
use self::thiserror::Error;
use crate::plumbing::{Pix, TesseractText};
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;

/// Wrapper around [`tesseract::TessBaseAPI`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html)
pub struct TessBaseAPI(*mut tesseract_sys::TessBaseAPI);

impl Drop for TessBaseAPI {
    fn drop(&mut self) {
        unsafe { TessBaseAPIDelete(self.0) }
    }
}

impl Default for TessBaseAPI {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error)]
#[error("TessBaseApi failed to initialize")]
pub struct TessBaseAPIInitError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to set variable")]
pub struct TessBaseAPISetVariableError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to recognize")]
pub struct TessBaseAPIRecogniseError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_utf8_text returned null")]
pub struct TessBaseAPIGetUTF8TextError();

#[derive(Debug, Error, PartialEq)]
pub enum TessBaseAPISetImageSafetyError {
    #[error("Image dimensions exceed computer memory")]
    DimensionsExceedMemory(),
    #[error("Image dimensions exceed image size")]
    DimensionsExceedImageSize(),
    #[error("Image width exceeds bytes per line")]
    ImageWidthExceedsBytesPerLine(),
}

impl TessBaseAPI {
    pub fn new() -> TessBaseAPI {
        TessBaseAPI(unsafe { TessBaseAPICreate() })
    }

    /// Wrapper for [`Init-2`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a965ef2ff51c440756519a3d6f755f34f)
    /// and [`TessBaseAPIInit3`](https://tesseract-ocr.github.io/tessapi/5.x/a00008.html#a5978ca2bd29df32a2eece528329ed425)
    ///
    /// Start tesseract
    pub fn init_2(
        &mut self,
        datapath: Option<&CStr>,
        language: Option<&CStr>,
    ) -> Result<(), TessBaseAPIInitError> {
        let ret = unsafe {
            TessBaseAPIInit3(
                self.0,
                datapath.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                language.map(CStr::as_ptr).unwrap_or_else(ptr::null),
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(TessBaseAPIInitError {})
        }
    }

    /// Wrapper for [`SetImage-2`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a0c4c7f05fd58b3665b123232a05545ad)
    pub fn set_image_2(&mut self, pix: &Pix) {
        unsafe {
            TessBaseAPISetImage2(self.0, *pix.as_ref());
        }
    }

    /// Wrapper for [`SetImage-1`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#aa463622111f3b11d8fca5863709cc699)
    pub fn set_image_1(
        &mut self,
        image_data: &[u8],
        width: c_int,
        height: c_int,
        bytes_per_pixel: c_int,
        bytes_per_line: c_int,
    ) -> Result<(), TessBaseAPISetImageSafetyError> {
        let claimed_image_size: usize = (height * bytes_per_line)
            .try_into()
            .map_err(|_| TessBaseAPISetImageSafetyError::DimensionsExceedMemory())?;
        if claimed_image_size > image_data.len() {
            return Err(TessBaseAPISetImageSafetyError::DimensionsExceedImageSize());
        }
        match bytes_per_pixel {
            0 => {
                if width > bytes_per_line * 8 {
                    return Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine());
                }
            }
            _ => {
                if width * bytes_per_pixel > bytes_per_line {
                    return Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine());
                }
            }
        }
        unsafe {
            TessBaseAPISetImage(
                self.0,
                image_data.as_ptr(),
                width,
                height,
                bytes_per_pixel,
                bytes_per_line,
            );
        };
        Ok(())
    }
    /// Wrapper for [`SetSourceResolution`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a4ded6137507a4e8eb6ed4bea0b9648f4)
    pub fn set_source_resolution(&mut self, ppi: c_int) {
        unsafe {
            TessBaseAPISetSourceResolution(self.0, ppi);
        }
    }

    /// Wrapper for [`SetVariable`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a2e09259c558c6d8e0f7e523cbaf5adf5)
    pub fn set_variable(
        &mut self,
        name: &CStr,
        value: &CStr,
    ) -> Result<(), TessBaseAPISetVariableError> {
        let ret = unsafe { TessBaseAPISetVariable(self.0, name.as_ptr(), value.as_ptr()) };
        match ret {
            1 => Ok(()),
            _ => Err(TessBaseAPISetVariableError {}),
        }
    }
    /// Wrapper for [`Recognize`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a0e4065c20b142d69a2324ee0c74ae0b0)
    ///
    /// Recognize the image. Returns `Ok(())` on success and `Err(())` otherwise.
    /// It is currently unclear to me what would make it error.
    ///
    /// It could take a progress argument (`monitor`). If there is appetite for this, let me know and I could try and implement it.
    pub fn recognize(&mut self) -> Result<(), TessBaseAPIRecogniseError> {
        let ret = unsafe { TessBaseAPIRecognize(self.0, ptr::null_mut()) };
        match ret {
            0 => Ok(()),
            _ => Err(TessBaseAPIRecogniseError {}),
        }
    }
    /// Wrapper for [`GetUTF8Text`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a115ef656f83352ba608b4f0bf9cfa2c4)
    ///
    /// Get the text out of an image.
    ///
    /// Can return an error (null pointer), but it is not clear to me what would cause this.
    ///
    /// This will implicitly call `recognize` if required.
    pub fn get_utf8_text(&mut self) -> Result<TesseractText, TessBaseAPIGetUTF8TextError> {
        let ptr = unsafe { TessBaseAPIGetUTF8Text(self.0) };
        if ptr.is_null() {
            Err(TessBaseAPIGetUTF8TextError {})
        } else {
            Ok(unsafe { TesseractText::new(ptr) })
        }
    }
}

#[test]
fn set_image_1_safety_test() -> Result<(), TessBaseAPIInitError> {
    let mut tess = TessBaseAPI::new();
    tess.init_2(None, None)?;
    assert_eq!(
        tess.set_image_1(include_bytes!("../../img.tiff"), 2256, 324, 3, 2256 * 3),
        Ok(())
    );
    assert_eq!(tess.set_image_1(&[0, 0, 0, 0], 2, 2, 1, 2), Ok(()));
    assert_eq!(
        tess.set_image_1(&[0, 0, 0], 2, 2, 1, 2),
        Err(TessBaseAPISetImageSafetyError::DimensionsExceedImageSize())
    );
    assert_eq!(
        tess.set_image_1(&[0, 0, 0, 0], 2, 2, 1, 1),
        Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine())
    );
    assert_eq!(tess.set_image_1(&[0, 0, 0, 0], 16, 2, 0, 2), Ok(()));
    assert_eq!(
        tess.set_image_1(&[0, 0, 0, 0], 17, 2, 0, 2),
        Err(TessBaseAPISetImageSafetyError::ImageWidthExceedsBytesPerLine())
    );
    Ok(())
}

#[test]
fn set_variable_error_test() -> Result<(), Box<dyn std::error::Error>> {
    let fail = std::ffi::CString::new("fail")?;
    let mut tess = TessBaseAPI::new();
    tess.init_2(None, None)?;
    assert!(tess.set_variable(&fail, &fail).is_err());
    Ok(())
}
