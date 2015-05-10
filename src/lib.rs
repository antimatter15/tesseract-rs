extern crate tesseract_sys;
extern crate libc;

use tesseract_sys::*;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::ffi::CStr;


pub struct Tesseract {
	raw: *mut TessBaseAPI
}

impl Tesseract {
	pub fn new() -> Tesseract {
		Tesseract {
			raw: unsafe { TessBaseAPICreate() }
		}
	}
	pub fn set_lang(&self, language: &str) -> i32 {
		unsafe { TessBaseAPIInit3(self.raw, ptr::null(), CString::new(language).unwrap().as_ptr()) }
	}
	pub fn set_image(&self, filename: &str) {
		unsafe {
			let img = pixRead(CString::new(filename).unwrap().as_ptr());
			TessBaseAPISetImage2(self.raw, img);
		}
	}
	pub fn recognize(&self) -> i32 {
		unsafe {
			TessBaseAPIRecognize(self.raw, ptr::null())
		}
	}
	pub fn get_text(&self) -> &str {
		unsafe {
			str::from_utf8(CStr::from_ptr(TessBaseAPIGetUTF8Text(self.raw)).to_bytes()).unwrap()
		}
	}
}

pub fn ocr(filename: &str, language: &str) -> String {
	let cube = Tesseract::new();
	cube.set_lang(language);
	cube.set_image(filename);
	cube.recognize();
	return cube.get_text().to_string()
}


#[test]
fn blah(){
	ocr("img.png", "eng");
}

#[test]
fn it_works() {
	let cube = Tesseract::new();
	cube.set_lang("eng");
	cube.set_image("img.png");
	cube.recognize();
	println!("{:?}", cube.get_text());
}
