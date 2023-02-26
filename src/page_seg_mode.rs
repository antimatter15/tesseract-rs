// ⚠️ This file is generated
// ⚠️ Regenerate with `make src/page_seg_mode.rs`

use tesseract_sys::TessPageSegMode;

/// Enum representing different PageSegMode options accepted by Tesseract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSegMode {
    /// Orientation and script detection only.
    PsmOsdOnly,
    /// Automatic page segmentation with orientation and script detection. (OSD)
    PsmAutoOsd,
    /// Automatic page segmentation, but no OSD, or OCR.
    PsmAutoOnly,
    /// Fully automatic page segmentation, but no OSD.
    PsmAuto,
    /// Assume a single column of text of variable sizes.
    PsmSingleColumn,
    /// Assume a single uniform block of vertically aligned text.
    PsmSingleBlockVertText,
    /// Assume a single uniform block of text. (Default.)
    PsmSingleBlock,
    /// Treat the image as a single text line.
    PsmSingleLine,
    /// Treat the image as a single word.
    PsmSingleWord,
    /// Treat the image as a single word in a circle.
    PsmCircleWord,
    /// Treat the image as a single character.
    PsmSingleChar,
    /// Find as much text as possible in no particular order.
    PsmSparseText,
    /// Sparse text with orientation and script det.
    PsmSparseTextOsd,
    /// Treat the image as a single text line, bypassing hacks that are Tesseract-specific.
    PsmRawLine,
}

impl PageSegMode {
    /// Get the page-seg-mode's value as used by Tesseract
    pub fn as_tess_page_seg_mode(&self) -> TessPageSegMode {
        match self {
            PageSegMode::PsmOsdOnly => tesseract_sys::TessPageSegMode_PSM_OSD_ONLY,
            PageSegMode::PsmAutoOsd => tesseract_sys::TessPageSegMode_PSM_AUTO_OSD,
            PageSegMode::PsmAutoOnly => tesseract_sys::TessPageSegMode_PSM_AUTO_ONLY,
            PageSegMode::PsmAuto => tesseract_sys::TessPageSegMode_PSM_AUTO,
            PageSegMode::PsmSingleColumn => tesseract_sys::TessPageSegMode_PSM_SINGLE_COLUMN,
            PageSegMode::PsmSingleBlockVertText => {
                tesseract_sys::TessPageSegMode_PSM_SINGLE_BLOCK_VERT_TEXT
            }
            PageSegMode::PsmSingleBlock => tesseract_sys::TessPageSegMode_PSM_SINGLE_BLOCK,
            PageSegMode::PsmSingleLine => tesseract_sys::TessPageSegMode_PSM_SINGLE_LINE,
            PageSegMode::PsmSingleWord => tesseract_sys::TessPageSegMode_PSM_SINGLE_WORD,
            PageSegMode::PsmCircleWord => tesseract_sys::TessPageSegMode_PSM_CIRCLE_WORD,
            PageSegMode::PsmSingleChar => tesseract_sys::TessPageSegMode_PSM_SINGLE_CHAR,
            PageSegMode::PsmSparseText => tesseract_sys::TessPageSegMode_PSM_SPARSE_TEXT,
            PageSegMode::PsmSparseTextOsd => tesseract_sys::TessPageSegMode_PSM_SPARSE_TEXT_OSD,
            PageSegMode::PsmRawLine => tesseract_sys::TessPageSegMode_PSM_RAW_LINE,
        }
    }
}
