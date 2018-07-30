extern crate stb;

use stb::image::{Image, Rgb};

fn main() {
    let image: Image<Rgb, u8> = Image::from_file("assets/lenna.png").unwrap();

    assert_eq!(512, image.width());
    assert_eq!(512, image.height());

    println!("{:?}", image.pixels().next());

    for (r, g, b) in image.pixels() {
        //println!("{}, {}, {}", r, g, b);
    }
}
