use tesseract::iterator::BlockResult;
use tesseract::{iterator::TextlineResult, Tesseract};
use tesseract::{TesseractError, TesseractIteratorResult};
use tesseract_sys::PolyBlockType;

fn init_tesseract(path: &str) -> Result<Tesseract, TesseractError> {
    let tesseract = Tesseract::new(None, Some("eng"))?;
    let tesseract = tesseract.set_image(path).expect("Failed to set image");
    Ok(tesseract)
}

#[test]
fn iterate_textlines() -> Result<(), TesseractError> {
    let tesseract = init_tesseract("./img.png")?;
    let mut tesseract = tesseract.recognize()?;

    let mut tesseract_iterator = tesseract.iterator().unwrap();
    let text = include_str!("../img.txt");
    let mut lines = text.lines();
    while let Some(textline) = tesseract_iterator.next::<TextlineResult>(None) {
        assert_eq!(PolyBlockType::PT_FLOWING_TEXT, textline.block_type());
        assert_eq!(
            textline
                .get_text()
                .expect("Textline had no text")
                .to_string()
                .trim(),
            lines
                .next()
                .expect("expected image text had fewer lines than detected in image")
        );
    }
    Ok(())
}

#[test]
fn iterate_block() -> Result<(), TesseractError> {
    let tesseract = init_tesseract("./img.png")?;
    let mut tesseract = tesseract.recognize()?;

    let mut tesseract_iterator = tesseract.iterator().unwrap();
    let text = include_str!("../img.txt");
    while let Some(textline) = tesseract_iterator.next::<BlockResult>(None) {
        assert_eq!(PolyBlockType::PT_FLOWING_TEXT, textline.block_type());
        assert_eq!(
            textline
                .get_text()
                .expect("Textline had no text")
                .to_string(),
            text
        );
    }
    Ok(())
}
