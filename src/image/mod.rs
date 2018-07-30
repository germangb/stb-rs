pub mod format;
pub mod ffi {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/stb_image.rs"));
}

use std::marker::PhantomData;

use std::ffi::CStr;
use std::fs;
use std::path::Path;

use std::iter::Cloned;
use std::slice;

use {Error, Result};

use self::format::{PixelData, PixelFormat};

/*
pub use self::format::{
    R16, R32f, R8, Rg16, Rg32f, Rg8, Rgb16, Rgb32f, Rgb8, Rgba16, Rgba32f, Rgba8,
};
*/

pub use self::format::{Rg, Rgb, Rgba, R};

pub struct Pixels<'a, F, D>
where
    D: 'a + PixelData,
    F: 'a + PixelFormat<D>,
{
    inner: Cloned<slice::Iter<'a, F::Item>>,
    total: usize,
}

impl<'a, F, D> ExactSizeIterator for Pixels<'a, F, D>
where
    D: 'a + PixelData,
    F: 'a + PixelFormat<D>,
{}

impl<'a, F, D> Iterator for Pixels<'a, F, D>
where
    D: 'a + PixelData,
    F: 'a + PixelFormat<D>,
{
    type Item = F::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.total, Some(self.total))
    }
}

pub struct Image<P: PixelFormat<D>, D: PixelData> {
    width: usize,
    height: usize,
    data: *mut P,
    _ph: PhantomData<D>,
}

unsafe impl<P, D> Send for Image<P, D>
where
    P: PixelFormat<D>,
    D: PixelData,
{
}

impl<P, D> Drop for Image<P, D>
where
    D: PixelData,
    P: PixelFormat<D>,
{
    fn drop(&mut self) {
        unsafe { ffi::stbi_image_free(self.data as _) }
    }
}

impl<F, D> Image<F, D>
where
    D: PixelData,
    F: PixelFormat<D>,
{
    #[inline]
    pub fn channels(&self) -> usize {
        F::size()
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn as_ptr(&self) -> *const F {
        self.data
    }

    pub fn as_slice(&self) -> &[F::Item] {
        unsafe {
            let s = self.width * self.height;
            slice::from_raw_parts(self.data as *const F::Item, s)
        }
    }

    pub fn pixels(&self) -> Pixels<F, D> {
        Pixels {
            inner: self.as_slice().iter().cloned(),
            total: self.width * self.height,
        }
    }

    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Self::from_memory(fs::read(path)?)
    }

    pub fn from_memory<M>(memory: M) -> Result<Self>
    where
        M: AsRef<[u8]>,
    {
        let data = memory.as_ref();
        let mut width = 0;
        let mut height = 0;
        let mut channels = 0;

        unsafe {
            let image_data = D::load_from_memory(
                data.as_ptr(),
                data.len() as _,
                &mut width,
                &mut height,
                &mut channels,
                F::size() as _,
            );

            if image_data.is_null() {
                let failure_reason = CStr::from_ptr(ffi::stbi_failure_reason() as _)
                    .to_string_lossy()
                    .into_owned();

                return Err(Error::Stb(failure_reason));
            }

            let width = width as usize;
            let height = height as usize;

            Ok(Image {
                width,
                height,
                data: image_data as _,
                _ph: PhantomData,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use image::*;

    macro_rules! test_type {
        ($($type:ty => $ch:expr),+) => {
            #[test]
            fn test() {
                $(
                    let im = Image::<$type>::from_file("assets/lenna.png").unwrap();
                    assert_eq!(512, im.width());
                    assert_eq!(512, im.height());
                    assert_eq!($ch, im.channels());
                    assert!(Image::<$type>::from_file(".").is_err());
                )+
            }
        }
    }

    test_type! {
        R8 => 1,
        Rg8 => 2,
        Rgb8 => 3,
        Rgba8 => 4,
        R16 => 1,
        Rg16 => 2,
        Rgb16 => 3,
        Rgba16 => 4,
        R32f => 1,
        Rg32f => 2,
        Rgb32f => 3,
        Rgba32f => 4
    }
}
