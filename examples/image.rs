extern crate stb;

use stb::image::{Image, Rgb32f};

use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    // load as 8bit rgb
    let image: Image<Rgb32f> = Image::from_file("assets/lenna.png")?;

    assert_eq!(512, image.width());
    assert_eq!(512, image.height());
    assert_eq!(3, image.channels());

    println!("{:?}", image.as_ptr());

    for (r, g, b) in image.pixels() {
        //println!("{}, {}, {}", r, g, b);
    }

    Ok(())
}
