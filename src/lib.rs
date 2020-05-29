extern crate thiserror;

use self::thiserror::Error;
use std::ffi::CString;
use std::ffi::NulError;
use std::str;

pub mod plumbing;

#[derive(Debug, Error)]
pub enum SetLangError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("TessBaseApi failed to initialize")]
    TessBaseAPIInitError(#[from] plumbing::TessBaseAPIInitError),
}

#[derive(Debug, Error)]
pub enum SetImageError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("Failed to read image")]
    PixReadError(#[from] plumbing::PixReadError),
}

#[derive(Debug, Error)]
pub enum SetVariableError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("TessBaseApi failed to set variable")]
    TessBaseAPISetVariableError(#[from] plumbing::TessBaseAPISetVariableError),
}

#[derive(Debug, Error)]
pub enum TesseractError {
    #[error("Failed to set language")]
    SetLangError(#[from] SetLangError),
    #[error("Failed to set image")]
    SetImageError(#[from] SetImageError),
    #[error("Errored whilst recognizing")]
    RecognizeError(#[from] plumbing::TessBaseAPIRecogniseError),
    #[error("Errored whilst getting text")]
    GetTextError(#[from] plumbing::TessBaseAPIGetUTF8TextError),
    #[error("Errored whilst setting frame")]
    SetFrameError(#[from] plumbing::TessBaseAPISetImageSafetyError),
    #[error("Errored whilst setting image from mem")]
    SetImgFromMemError(#[from] plumbing::PixReadMemError),
    #[error("Errored whilst setting variable")]
    SetVariableError(#[from] SetVariableError),
}

pub struct Tesseract(plumbing::TessBaseAPI);

impl Default for Tesseract {
    fn default() -> Self {
        Self::new()
    }
}

impl Tesseract {
    pub fn new() -> Tesseract {
        Tesseract(plumbing::TessBaseAPI::new())
    }
    pub fn set_lang(&mut self, language: &str) -> Result<(), SetLangError> {
        Ok(self.0.init_2(None, Some(&CString::new(language)?))?)
    }
    pub fn set_image(&mut self, filename: &str) -> Result<(), SetImageError> {
        let pix = plumbing::Pix::read(&CString::new(filename)?)?;
        self.0.set_image_2(&pix);
        Ok(())
    }
    pub fn set_frame(
        &mut self,
        frame_data: &[u8],
        width: i32,
        height: i32,
        bytes_per_pixel: i32,
        bytes_per_line: i32,
    ) -> Result<(), plumbing::TessBaseAPISetImageSafetyError> {
        self.0
            .set_image_1(frame_data, width, height, bytes_per_pixel, bytes_per_line)
    }
    pub fn set_image_from_mem(&mut self, img: &[u8]) -> Result<(), plumbing::PixReadMemError> {
        let pix = plumbing::Pix::read_mem(img)?;
        self.0.set_image_2(&pix);
        Ok(())
    }

    pub fn set_source_resolution(&mut self, ppi: i32) {
        self.0.set_source_resolution(ppi)
    }

    pub fn set_variable(&mut self, name: &str, value: &str) -> Result<(), SetVariableError> {
        Ok(self
            .0
            .set_variable(&CString::new(name)?, &CString::new(value)?)?)
    }
    pub fn recognize(&mut self) -> Result<(), plumbing::TessBaseAPIRecogniseError> {
        self.0.recognize()
    }
    pub fn get_text(&mut self) -> Result<String, plumbing::TessBaseAPIGetUTF8TextError> {
        Ok(self
            .0
            .get_utf8_text()?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }
}

pub fn ocr(filename: &str, language: &str) -> Result<String, TesseractError> {
    let mut cube = Tesseract::new();
    cube.set_lang(language)?;
    cube.set_image(filename)?;
    cube.recognize()?;
    Ok(cube.get_text()?)
}

pub fn ocr_from_frame(
    frame_data: &[u8],
    width: i32,
    height: i32,
    bytes_per_pixel: i32,
    bytes_per_line: i32,
    language: &str,
) -> Result<String, TesseractError> {
    let mut cube = Tesseract::new();
    cube.set_lang(language)?;
    cube.set_frame(frame_data, width, height, bytes_per_pixel, bytes_per_line)?;
    cube.recognize()?;
    Ok(cube.get_text()?)
}

#[test]
fn ocr_test() -> Result<(), TesseractError> {
    assert_eq!(
        ocr("img.png", "eng")?,
        include_str!("../img.txt").to_string()
    );
    Ok(())
}

#[test]
fn ocr_from_frame_test() -> Result<(), TesseractError> {
    assert_eq!(
        ocr_from_frame(include_bytes!("../img.tiff"), 2256, 324, 3, 2256 * 3, "eng")?,
        include_str!("../img.txt").to_string()
    );
    Ok(())
}

#[test]
fn ocr_from_mem_with_ppi() -> Result<(), TesseractError> {
    let mut cube = Tesseract::new();
    cube.set_lang("eng")?;
    cube.set_image_from_mem(include_bytes!("../img.tiff"))?;

    cube.set_source_resolution(70);
    assert_eq!(&cube.get_text()?, include_str!("../img.txt"));
    Ok(())
}

#[test]
fn expanded_test() -> Result<(), TesseractError> {
    let mut cube = Tesseract::new();
    cube.set_lang("eng")?;
    cube.set_image("img.png")?;
    cube.set_variable("tessedit_char_blacklist", "z")?;
    cube.recognize()?;
    assert_eq!(&cube.get_text()?, include_str!("../img.txt"));
    Ok(())
}
