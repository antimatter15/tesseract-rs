//! A direct but safe wrapper for `tesseract-sys`. It should stick as close as
//! possible to the upstream API whilst avoiding unsafe behaviour.
//!
//! Are you interested in using this on its own?
//! Raise an issue, and I'll split it into its own crate.
mod pix;
mod tess_base_api;
mod tesseract_text;

pub use self::pix::Pix;
pub use self::pix::PixReadError;
pub use self::pix::PixReadMemError;
pub use self::tess_base_api::TessBaseAPI;
pub use self::tess_base_api::TessBaseAPIGetUTF8TextError;
pub use self::tess_base_api::TessBaseAPIInitError;
pub use self::tess_base_api::TessBaseAPIRecogniseError;
pub use self::tess_base_api::TessBaseAPISetImageSafetyError;
pub use self::tess_base_api::TessBaseAPISetVariableError;
pub use self::tesseract_text::TesseractText;

#[test]
fn ocr_from_mem_with_ppi() -> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::CString;

    let pix = Pix::read_mem(include_bytes!("../../img.tiff"))?;

    let mut cube = TessBaseAPI::new();
    cube.init_2(None, Some(&CString::new("eng")?))?;
    cube.set_image_2(&pix);

    cube.set_source_resolution(70);
    assert_eq!(
        cube.get_utf8_text()?.as_ref().to_str()?,
        include_str!("../../img.txt")
    );
    Ok(())
}

#[test]
fn expanded_test() -> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::CString;

    let mut cube = TessBaseAPI::new();
    cube.set_variable(
        &CString::new("tessedit_char_blacklist")?,
        &CString::new("z")?,
    )?;
    cube.init_2(None, None)?;
    let pix = Pix::read(&CString::new("../img.png")?)?;
    cube.set_image_2(&pix);
    cube.recognize()?;
    assert_eq!(
        cube.get_utf8_text()?.as_ref().to_str()?,
        include_str!("../../img.txt")
    );
    Ok(())
}

#[test]
fn setting_image_without_initializing_test() -> Result<(), PixReadMemError> {
    let mut cube = TessBaseAPI::new();
    let pix = Pix::read_mem(include_bytes!("../../img.tiff"))?;
    cube.set_image_2(&pix);
    assert!(cube.recognize().is_err());
    assert!(cube.get_utf8_text().is_err());
    Ok(())
}
