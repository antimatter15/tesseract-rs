pub mod plumbing;

use std::ffi::CString;
use std::str;

pub struct Tesseract(plumbing::TessBaseAPI);

impl Default for Tesseract {
    fn default() -> Self {
        Self::new()
    }
}

fn cs(string: &str) -> CString {
    CString::new(string).unwrap()
}

impl Tesseract {
    pub fn new() -> Tesseract {
        Tesseract(plumbing::TessBaseAPI::new())
    }
    pub fn set_lang(&mut self, language: &str) -> i32 {
        let cs_language = cs(language);
        match self.0.init_2(None, Some(&cs_language)) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
    pub fn set_image(&mut self, filename: &str) {
        let cs_filename = cs(filename);
        let img = plumbing::Pix::read(&cs_filename).unwrap();
        self.0.set_image_2(&img);
    }
    pub fn set_frame(
        &mut self,
        frame_data: &[u8],
        width: i32,
        height: i32,
        bytes_per_pixel: i32,
        bytes_per_line: i32,
    ) {
        self.0
            .set_image_1(frame_data, width, height, bytes_per_pixel, bytes_per_line)
            .unwrap();
    }
    pub fn set_image_from_mem(&mut self, img: &[u8]) {
        let pix = plumbing::Pix::read_mem(img).unwrap();
        self.0.set_image_2(&pix);
    }

    pub fn set_source_resolution(&mut self, ppi: i32) {
        self.0.set_source_resolution(ppi)
    }

    pub fn set_variable(&mut self, name: &str, value: &str) -> i32 {
        let cs_name = cs(name);
        let cs_value = cs(value);
        match self.0.set_variable(&cs_name, &cs_value) {
            Ok(()) => 1,
            Err(_) => 0,
        }
    }
    pub fn recognize(&mut self) -> i32 {
        match self.0.recognize() {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
    pub fn get_text(&mut self) -> String {
        self.0
            .get_utf8_text()
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned()
    }
}

pub fn ocr(filename: &str, language: &str) -> String {
    let mut cube = Tesseract::new();
    cube.set_lang(language);
    cube.set_image(filename);
    cube.recognize();
    cube.get_text()
}

pub fn ocr_from_frame(
    frame_data: &[u8],
    width: i32,
    height: i32,
    bytes_per_pixel: i32,
    bytes_per_line: i32,
    language: &str,
) -> String {
    let mut cube = Tesseract::new();
    cube.set_lang(language);
    cube.set_frame(frame_data, width, height, bytes_per_pixel, bytes_per_line);
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
fn ocr_from_frame_test() {
    use std::fs::File;
    use std::io::Read;

    let mut img = File::open("img.tiff").unwrap();
    let mut buffer = Vec::new();
    img.read_to_end(&mut buffer).unwrap();

    assert_eq!(
        ocr_from_frame(&buffer, 2256, 324, 3, 2256 * 3, "eng"),
        include_str!("../img.txt").to_string()
    );
}

#[test]
fn ocr_from_mem_with_ppi() {
    use std::fs::File;
    use std::io::Read;

    let mut img = File::open("img.tiff").unwrap();
    let mut buffer = Vec::new();
    img.read_to_end(&mut buffer).unwrap();

    let mut cube = Tesseract::new();
    cube.set_lang("eng");
    cube.set_image_from_mem(&buffer);

    cube.set_source_resolution(70);
    assert_eq!(cube.get_text(), include_str!("../img.txt").to_string());
}

#[test]
fn expanded_test() {
    let mut cube = Tesseract::new();
    cube.set_lang("eng");
    cube.set_image("img.png");
    cube.recognize();
    assert_eq!(cube.get_text(), include_str!("../img.txt").to_string())
}
