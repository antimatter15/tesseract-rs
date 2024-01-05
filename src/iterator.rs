use tesseract_plumbing::Text;
use tesseract_sys::{PageIteratorLevel, PolyBlockType};

/// Iterate through the OCR results of an image
///
/// As the results are ephemeral, the `next` function only yields references to these results.
/// ```
/// use tesseract::{Tesseract, iterator::TextlineResult};
/// use crate::tesseract::TesseractIteratorResult;
/// use tesseract_sys::PolyBlockType;
///
/// let mut tesseract = Tesseract::new(None, Some("eng")).unwrap();
/// let mut tesseract = tesseract.set_image("./img.png").expect("Failed to set image");
/// let mut tesseract = tesseract.recognize().unwrap();
/// let mut tesseract_iterator = tesseract.iterator().unwrap();

/// while let Some(textline) = tesseract_iterator.next::<TextlineResult>(None) {
///     assert_eq!(PolyBlockType::PT_FLOWING_TEXT ,textline.block_type());
/// }
/// ```
pub struct ResultIterator(tesseract_plumbing::ResultIterator, bool);

pub struct BlockResult<'a>(&'a tesseract_plumbing::ResultIterator);
pub struct ParagraphResult<'a>(&'a tesseract_plumbing::ResultIterator);
pub struct TextlineResult<'a>(&'a tesseract_plumbing::ResultIterator);
pub struct WordResult<'a>(&'a tesseract_plumbing::ResultIterator);
pub struct SymbolResult<'a>(&'a tesseract_plumbing::ResultIterator);

impl ResultIterator {
    pub(crate) fn new(iterator: tesseract_plumbing::ResultIterator) -> Self {
        Self(iterator, true)
    }
    pub fn next<'a, T>(&'a mut self, limit: Option<PageIteratorLevel>) -> Option<T>
    where
        T: TesseractIteratorResult<'a>,
    {
        let p = *self.0.as_ref();
        if self.1 {
            self.1 = false;
            return Some(T::from(&self.0));
        }
        let end_of_page_reached =
            unsafe { tesseract_sys::TessPageIteratorNext(p.cast(), T::LEVEL as u32) == 0 };
        if end_of_page_reached {
            return None;
        }
        Some(T::from(&self.0))
    }
}

pub trait TesseractIteratorResult<'a>
where
    Self: From<&'a tesseract_plumbing::ResultIterator>
        + AsRef<tesseract_plumbing::ResultIterator>
        + 'a,
{
    /// The equivalent PageIteratorLevel of this result
    const LEVEL: PageIteratorLevel;
    /// Get the text contained of the iteration result
    fn get_text(&self) -> Option<Text> {
        let c_str = unsafe {
            tesseract_sys::TessResultIteratorGetUTF8Text(
                self.as_ref().as_ref().cast(),
                Self::LEVEL as u32,
            )
        };
        if c_str.is_null() {
            return None;
        }
        Some(unsafe { tesseract_plumbing::Text::new(c_str) })
    }

    /// Get the bounding box of the iteration result
    fn bounding_box(&self) -> BoundingBox {
        let mut left = 0;
        let mut right = 0;
        let mut top = 0;
        let mut bottom = 0;
        // TODO: Use this to verify
        let _object_at_pos = unsafe {
            tesseract_sys::TessPageIteratorBoundingBox(
                self.as_ref().as_ref().cast(),
                Self::LEVEL as u32,
                &mut left,
                &mut top,
                &mut right,
                &mut bottom,
            )
        };
        BoundingBox {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Check what the current block type is
    fn block_type(&self) -> PolyBlockType {
        let block_type =
            unsafe { tesseract_sys::TessPageIteratorBlockType(self.as_ref().as_ref().cast()) };
        unsafe { std::mem::transmute(block_type) } // TODO: This doesn't check that the value is valid
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

/// All results implement the same basic functionality, but with slight differences in the PageIteratorLevel
macro_rules! result_impls {
    ($name:ident -> $level:expr $(, $($tts:tt)*)?) => {

        impl<'a> TesseractIteratorResult<'a> for $name<'a>
        where Self: From<&'a tesseract_plumbing::ResultIterator> + AsRef<tesseract_plumbing::ResultIterator> + 'a
        {
            const LEVEL: PageIteratorLevel = $level;
        }

        impl<'a> From<&'a tesseract_plumbing::ResultIterator> for $name<'a> {
            fn from(value: &'a tesseract_plumbing::ResultIterator) -> $name<'a> {
                $name(value)
            }
        }

        impl AsRef<tesseract_plumbing::ResultIterator> for $name<'_> {
            fn as_ref(&self) -> &tesseract_plumbing::ResultIterator {
                self.0
            }
        }

        result_impls!($($($tts)*)?);
    };
    () => {};
}

result_impls!(
    BlockResult -> PageIteratorLevel::RIL_BLOCK,
    ParagraphResult -> PageIteratorLevel::RIL_PARA,
    TextlineResult -> PageIteratorLevel::RIL_TEXTLINE,
    WordResult -> PageIteratorLevel::RIL_WORD,
    SymbolResult -> PageIteratorLevel::RIL_SYMBOL
);
