#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use leptonica_sys::{Boxa, Pix, Pixa, _IO_FILE};

include!(concat!(env!("OUT_DIR"), "/capi_bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/public_types_bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use leptonica_sys::{pixFreeData, pixRead};
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::ptr;
    use std::str;

    #[test]
    fn ocr() {
        unsafe {
            let cube = TessBaseAPICreate();
            TessBaseAPIInit3(cube, ptr::null(), CString::new("eng").unwrap().as_ptr());
            let image = pixRead(CString::new("../img.png").unwrap().as_ptr());
            TessBaseAPISetImage2(cube, image);
            TessBaseAPIRecognize(cube, ptr::null_mut());
            assert_eq!(
                str::from_utf8(CStr::from_ptr(TessBaseAPIGetUTF8Text(cube)).to_bytes()),
                Ok(include_str!("../../img.txt"))
            );
            pixFreeData(image);
            TessBaseAPIDelete(cube);
        }
    }

    #[test]
    #[allow(path_statements)]
    fn defined_constants() {
        kMinCredibleResolution;
        kMaxCredibleResolution;
    }
}
