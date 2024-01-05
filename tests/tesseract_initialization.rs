use tesseract::{ocr, ocr_from_frame, OcrEngineMode, Tesseract, TesseractError};

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
#[ignore = "Many systems do not have legacy Tesseract data available"]
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

#[test]
fn oem_ltsm_only_test() -> Result<(), TesseractError> {
    let only_lstm_str = Tesseract::new_with_oem(None, Some("eng"), OcrEngineMode::LstmOnly)?
        .set_image("img.png")?
        .recognize()?
        .get_text()?;

    assert_eq!(only_lstm_str, include_str!("../img.txt"));
    Ok(())
}

#[test]
fn initialize_with_none() {
    assert!(Tesseract::new(None, None).is_ok());
}
