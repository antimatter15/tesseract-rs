extern crate tesseract;
use tesseract::*;

fn main() {
    println!("Hello, world!");
    let mut cube = Tesseract::new();
    let filename = "img.png";
    let language = "eng";
	cube.set_lang(language);
	cube.set_image(filename);
	cube.set_variable("save_best_choices", "T");
	cube.recognize();

	println!("{}", cube.get_text().to_string());

    // println!("{:?}", ocr("img.png", "eng"));
}
