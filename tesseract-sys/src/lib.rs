#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
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
}
