use std::env;
use std::path::PathBuf;
use bindgen;

fn main() {
    println!("cargo:rustc-link-lib=x52");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libx52_binding.rs"))
        .expect("Couldn't write bindings!");
}