extern crate bindgen;
extern crate cc;

use std::{path, env, fs};

macro_rules! out {
    ($path:tt) => {{
        let mut path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
        path.push($path);
        path
    }}
}

fn build_stb_image() {
    fs::copy("stb/stb_image.h", out!("stb_image.c").to_str().unwrap())
        .expect("Error copying stb_image.h -> stb_image.c");

    cc::Build::new()
        .file(out!("stb_image.c").to_str().unwrap())
        .define("STB_IMAGE_IMPLEMENTATION", None)

        //TODO support defines
        .compile("stb_image");

    //println!("cargo:rustc-link-lib=stb_image");

    // gen bindings
    bindgen::builder()
        .header("stb/stb_image.h")

        .whitelist_function("stbi_(.*)")
        .whitelist_type("stbi_(.*)")

        .layout_tests(false)
        .rustfmt_bindings(true)

        .generate()
        .expect("Error generating bindings!!")
        
        // write to disk
        .write_to_file(out!("stb_image.rs").to_str().unwrap())
        .expect("Error writing bindings!!");
}

fn main() {
    build_stb_image();
}
