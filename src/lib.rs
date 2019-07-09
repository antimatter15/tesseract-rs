extern crate leptonica_sys;
extern crate tesseract_sys;

use leptonica_sys::{pixFreeData, pixRead};
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use std::str;
use tesseract_sys::{
    TessBaseAPI, TessBaseAPICreate, TessBaseAPIDelete, TessBaseAPIGetUTF8Text, TessBaseAPIInit3,
    TessBaseAPIRecognize, TessBaseAPISetImage2, TessBaseAPISetVariable, TessDeleteText,
};

pub struct Tesseract {
    raw: *mut TessBaseAPI,
}

impl Drop for Tesseract {
    fn drop(&mut self) {
        unsafe { TessBaseAPIDelete(self.raw) }
    }
}

impl Default for Tesseract {
    fn default() -> Self {
        Self::new()
    }
}

fn cs(string: &str) -> CString {
    // do not call as_ptr yet, since the data will be freed before we return
    CString::new(string).unwrap()
}

impl Tesseract {
    pub fn new() -> Tesseract {
        Tesseract {
            raw: unsafe { TessBaseAPICreate() },
        }
    }
    pub fn set_lang(&mut self, language: &str) -> i32 {
        let cs_language = cs(language);
        unsafe { TessBaseAPIInit3(self.raw, ptr::null(), cs_language.as_ptr()) }
    }
    pub fn set_image(&mut self, filename: &str) {
        let cs_filename = cs(filename);
        unsafe {
            let img = pixRead(cs_filename.as_ptr());
            TessBaseAPISetImage2(self.raw, img);
            pixFreeData(img);
        }
    }
    pub fn set_variable(&mut self, name: &str, value: &str) -> i32 {
        let cs_name = cs(name);
        let cs_value = cs(value);
        unsafe { TessBaseAPISetVariable(self.raw, cs_name.as_ptr(), cs_value.as_ptr()) }
    }
    pub fn recognize(&mut self) -> i32 {
        unsafe { TessBaseAPIRecognize(self.raw, ptr::null_mut()) }
    }
    pub fn get_text(&self) -> String {
        unsafe {
            let cs_value = TessBaseAPIGetUTF8Text(self.raw);
            let string = CStr::from_ptr(cs_value).to_string_lossy().into_owned();
            TessDeleteText(cs_value);
            string
        }
    }
}

pub fn ocr(filename: &str, language: &str) -> String {
    let mut cube = Tesseract::new();
    cube.set_lang(language);
    cube.set_image(filename);
    cube.recognize();
    cube.get_text()
}

#[test]
fn ocr_test() {
    assert_eq!(
        ocr("img.png", "eng"),
        include_str!("../img.txt").to_string()
    );
}

#[test]
fn expanded_test() {
    let mut cube = Tesseract::new();
    cube.set_lang("eng");
    cube.set_image("img.png");
    cube.recognize();
    assert_eq!(cube.get_text(), include_str!("../img.txt").to_string())
}
