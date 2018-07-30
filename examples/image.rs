extern crate stb;

use stb::image::{Image, Rgb};

fn main() {
    let image: Image<Rgb, u8> = Image::from_file("assets/lenna.png").unwrap();

    assert_eq!(512, image.width());
    assert_eq!(512, image.height());

    println!("{:?}", image.texels().next());

    for (r, g, b) in image.texels() {
        //println!("{}, {}, {}", r, g, b);
    }
}
