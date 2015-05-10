extern crate libc;
use libc::{c_int, c_char};

#[link(name = "lept")]
extern {
    pub fn pixRead(filename: *const libc::c_char) -> *mut PIX;
}

#[link(name = "tesseract")]
extern {
    pub fn TessBaseAPICreate() -> *mut TessBaseAPI;
    pub fn TessBaseAPIInit3(handle: *mut TessBaseAPI, 
    						datapath: *const libc::c_char, 
    						language: *const libc::c_char) -> libc::c_int;
    pub fn TessBaseAPISetImage2(handle: *mut TessBaseAPI, 
    							pix: *const PIX);
    pub fn TessBaseAPIRecognize(handle: *mut TessBaseAPI, 
    							monitor: *const ETEXT_DESC) -> libc::c_int;
    pub fn TessBaseAPIGetUTF8Text(handle: *mut TessBaseAPI) -> *const libc::c_char;
}

#[repr(C)]
pub struct TessBaseAPI;

#[repr(C)]
pub struct ETEXT_DESC;

#[repr(C)]
pub struct PIX;

#[test]
fn it_works() {
}


