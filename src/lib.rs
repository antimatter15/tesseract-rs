extern crate thiserror;

use self::thiserror::Error;
use std::ffi::CString;
use std::ffi::NulError;
use std::os::raw::c_int;
use std::str;

pub mod plumbing;

#[derive(Debug, Error)]
pub enum InitializeError {
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
    InitializeError(#[from] InitializeError),
    #[error("Failed to set image")]
    SetImageError(#[from] SetImageError),
    #[error("Errored whilst recognizing")]
    RecognizeError(#[from] plumbing::TessBaseAPIRecogniseError),
    #[error("Errored whilst getting text")]
    GetTextError(#[from] plumbing::TessBaseAPIGetUTF8TextError),
    #[error("Errored whilst getting HOCR text")]
    GetHOCRTextError(#[from] plumbing::TessBaseAPIGetHOCRTextError),
    #[error("Errored whilst setting frame")]
    SetFrameError(#[from] plumbing::TessBaseAPISetImageSafetyError),
    #[error("Errored whilst setting image from mem")]
    SetImgFromMemError(#[from] plumbing::PixReadMemError),
    #[error("Errored whilst setting variable")]
    SetVariableError(#[from] SetVariableError),
}

pub enum OcrEngineMode {
    Default,
    LstmOnly,
    TesseractLstmCombined,
    TesseractOnly,
}

impl OcrEngineMode {
    pub(crate) fn to_value(&self) -> plumbing::TessOcrEngineMode {
        match *self {
            OcrEngineMode::Default => plumbing::TessOcrEngineMode_OEM_DEFAULT,
            OcrEngineMode::LstmOnly => plumbing::TessOcrEngineMode_OEM_LSTM_ONLY,
            OcrEngineMode::TesseractLstmCombined => {
                plumbing::TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED
            }
            OcrEngineMode::TesseractOnly => plumbing::TessOcrEngineMode_OEM_TESSERACT_ONLY,
        }
    }
}

pub struct Tesseract(plumbing::TessBaseAPI);

impl Tesseract {
    pub fn new(datapath: Option<&str>, language: Option<&str>) -> Result<Self, InitializeError> {
        let mut tess = Tesseract(plumbing::TessBaseAPI::new());
        let datapath = match datapath {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };
        let language = match language {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };

        tess.0.init_2(datapath.as_deref(), language.as_deref())?;
        Ok(tess)
    }

    pub fn new_with_oem(
        datapath: Option<&str>,
        language: Option<&str>,
        oem: OcrEngineMode,
    ) -> Result<Self, InitializeError> {
        let mut tess = Tesseract(plumbing::TessBaseAPI::new());
        let datapath = match datapath {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };
        let language = match language {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };

        tess.0
            .init_4(datapath.as_deref(), language.as_deref(), oem.to_value())?;
        Ok(tess)
    }

    pub fn set_image(mut self, filename: &str) -> Result<Self, SetImageError> {
        let pix = plumbing::Pix::read(&CString::new(filename)?)?;
        self.0.set_image_2(&pix);
        Ok(self)
    }
    pub fn set_frame(
        mut self,
        frame_data: &[u8],
        width: i32,
        height: i32,
        bytes_per_pixel: i32,
        bytes_per_line: i32,
    ) -> Result<Self, plumbing::TessBaseAPISetImageSafetyError> {
        self.0
            .set_image_1(frame_data, width, height, bytes_per_pixel, bytes_per_line)?;
        Ok(self)
    }
    pub fn set_image_from_mem(mut self, img: &[u8]) -> Result<Self, plumbing::PixReadMemError> {
        let pix = plumbing::Pix::read_mem(img)?;
        self.0.set_image_2(&pix);
        Ok(self)
    }

    pub fn set_source_resolution(mut self, ppi: i32) -> Self {
        self.0.set_source_resolution(ppi);
        self
    }

    pub fn set_variable(mut self, name: &str, value: &str) -> Result<Self, SetVariableError> {
        self.0
            .set_variable(&CString::new(name)?, &CString::new(value)?)?;
        Ok(self)
    }
    pub fn recognize(mut self) -> Result<Self, plumbing::TessBaseAPIRecogniseError> {
        self.0.recognize()?;
        Ok(self)
    }
    pub fn get_text(&mut self) -> Result<String, plumbing::TessBaseAPIGetUTF8TextError> {
        Ok(self
            .0
            .get_utf8_text()?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    /// Get the text encoded as HTML with bounding box tags
    ///
    /// See [img.html](../img.html) for an example.
    pub fn get_hocr_text(
        &mut self,
        page: c_int,
    ) -> Result<String, plumbing::TessBaseAPIGetHOCRTextError> {
        Ok(self
            .0
            .get_hocr_text(page)?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }
}

pub fn ocr(filename: &str, language: &str) -> Result<String, TesseractError> {
    Ok(Tesseract::new(None, Some(language))?
        .set_image(filename)?
        .recognize()?
        .get_text()?)
}

pub fn ocr_from_frame(
    frame_data: &[u8],
    width: i32,
    height: i32,
    bytes_per_pixel: i32,
    bytes_per_line: i32,
    language: &str,
) -> Result<String, TesseractError> {
    Ok(Tesseract::new(None, Some(language))?
        .set_frame(frame_data, width, height, bytes_per_pixel, bytes_per_line)?
        .recognize()?
        .get_text()?)
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
    let mut cube = Tesseract::new(None, Some("eng"))?
        .set_image_from_mem(include_bytes!("../img.tiff"))?
        .set_source_resolution(70);
    assert_eq!(&cube.get_text()?, include_str!("../img.txt"));
    Ok(())
}

#[test]
fn expanded_test() -> Result<(), TesseractError> {
    let mut cube = Tesseract::new(None, Some("eng"))?
        .set_image("img.png")?
        .set_variable("tessedit_char_blacklist", "z")?
        .recognize()?;
    assert_eq!(&cube.get_text()?, include_str!("../img.txt"));
    Ok(())
}

#[test]
fn hocr_test() -> Result<(), TesseractError> {
    let mut cube = Tesseract::new(None, Some("eng"))?.set_image("img.png")?;
    assert!(&cube.get_hocr_text(0)?.contains("<div class='ocr_page'"));
    Ok(())
}

#[test]
fn oem_test() -> Result<(), TesseractError> {
    let only_tesseract_str =
        Tesseract::new_with_oem(None, Some("eng"), OcrEngineMode::TesseractOnly)?
            .set_image("img.png")?
            .recognize()?
            .get_text()?;

    let only_lstm_str = Tesseract::new_with_oem(None, Some("eng"), OcrEngineMode::LstmOnly)?
        .set_image("img.png")?
        .recognize()?
        .get_text()?;

    assert_ne!(only_tesseract_str, only_lstm_str);
    Ok(())
}
