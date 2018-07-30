extern crate stb;

use stb::image::{Image, Rgb32f};

fn main() {
    let image: Image<Rgb32f> = Image::from_file("assets/lenna.png").unwrap();

    assert_eq!(512, image.width());
    assert_eq!(512, image.height());

    for (r, g, b) in image.pixels() {
        //println!("{}, {}, {}", r, g, b);
    }
}
