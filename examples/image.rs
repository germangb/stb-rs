extern crate stb;

use stb::image::Image;

use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    // load as 8bit rgb
    let image: Image<u8> = Image::from_reader(File::open("assets/lenna.png")?, 3)?;

    // load as f32 rgb
    let image: Image<f32> = Image::from_reader(File::open("assets/lenna.png")?, 3)?;

    Ok(())
}
