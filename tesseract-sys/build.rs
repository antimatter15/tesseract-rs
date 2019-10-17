extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system tesseract
    // and leptonica shared libraries.
    println!("cargo:rustc-link-lib=tesseract");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let capi_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper_capi.h")
        .whitelist_function("^Tess.*")
        .blacklist_type("Boxa")
        .blacklist_type("Pix")
        .blacklist_type("Pixa")
        .blacklist_type("_IO_FILE")
        .blacklist_type("_IO_codecvt")
        .blacklist_type("_IO_marker")
        .blacklist_type("_IO_wide_data")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate capi bindings");

    let public_types_bindings = bindgen::Builder::default()
        .header("wrapper_public_types.hpp")
        .whitelist_var("^k.*")
        .blacklist_item("kPolyBlockNames")
        .generate()
        .expect("Unable to generate public types bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    capi_bindings
        .write_to_file(out_path.join("capi_bindings.rs"))
        .expect("Couldn't write capi bindings!");
    public_types_bindings
        .write_to_file(out_path.join("public_types_bindings.rs"))
        .expect("Couldn't write public types bindings!");
}
