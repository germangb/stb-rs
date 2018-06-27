use std::ffi::CStr;
use std::path::Path;
use std::fs::File;

use std::io::Read;

use {Error, Result};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/stb_image.rs"));
}

use std::os::raw::c_int;

pub trait Data {
    unsafe fn from_memory(
        buffer: *const ffi::stbi_uc,
        len: c_int,
        x: &mut c_int,
        y: &mut c_int,
        c: &mut c_int,
        desired: c_int,
    ) -> *mut Self;
}

macro_rules! impl_traits {
    ($($type:ty => ($file:ident, $mem:ident),)+) => {
        $(
            impl Data for $type {
                unsafe fn from_memory(
                    buffer: *const ffi::stbi_uc,
                    len: c_int,
                    x: &mut c_int,
                    y: &mut c_int,
                    c: &mut c_int,
                    desired: c_int,
                ) -> *mut Self {
                    ffi::$mem(buffer, len, x, y, c, desired) as _
                }
            }
        )+
    }
}

impl_traits! {
    u8 => (stbi_load, stbi_load_from_memory),
    u16 => (stbi_load_16, stbi_load_16_from_memory),
    f32 => (stbi_loadf, stbi_loadf_from_memory),
}

#[derive(Debug)]
pub struct Image<S> {
    width: usize,
    height: usize,
    channels: usize,
    data: *mut S,
}

unsafe impl<S: Data> Send for Image<S> {}

impl<S> Drop for Image<S> {
    fn drop(&mut self) {
        unsafe { ffi::stbi_image_free(self.data as _) }
    }
}

impl<S: Data> Image<S> {}

impl<S> Image<S>
where
    S: Data,
{
    pub fn open<P: AsRef<Path>>(path: P, desired_channels: usize) -> Result<Self> {
        Self::from_reader(File::open(path)?, desired_channels)
    }

    pub fn from_reader<R: Read>(mut reader: R, desired_channels: usize) -> Result<Self> {
        let mut data = Vec::new();

        reader.read_to_end(&mut data)?;
        Self::from_memory(data, desired_channels)
    }

    pub fn from_memory<M>(memory: M, desired_channels: usize) -> Result<Image<S>>
    where
        M: AsRef<[u8]>,
    {
        let data = memory.as_ref();
        let mut width = 0;
        let mut height = 0;
        let mut channels = 0;

        unsafe {
            let image_data = S::from_memory(
                data.as_ptr(),
                data.len() as _,
                &mut width,
                &mut height,
                &mut channels,
                desired_channels as _,
            );

            if image_data.is_null() {
                let failure_reason = CStr::from_ptr(ffi::stbi_failure_reason() as _)
                    .to_string_lossy()
                    .into_owned();

                return Err(Error::Stb(failure_reason));
            }

            let width = width as usize;
            let height = height as usize;
            let channels = channels as usize;

            Ok(Image {
                width,
                height,
                channels,
                data: image_data,
            })
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const S {
        self.data
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
    pub fn channels(&self) -> usize {
        self.channels
    }
}

#[cfg(test)]
mod tests {
    use image::{ffi, Image};

    use std::ffi::CString;
    use std::os::raw;

    macro_rules! test_file {
        ($(fn $name_test:ident() => $test_fn:ident ;)*) => {
            $(
                #[test]
                fn $name_test() {
                    let image = ::image::$test_fn("assets/lenna.png", 3);

                    assert!(image.is_ok());

                    let image = image.unwrap();
                    assert_eq!(512, image.width());
                    assert_eq!(512, image.height());
                    assert_eq!(3, image.channels());

                    // load failure
                    let image = ::image::load("nope", 3);
                    assert!(image.is_err());
                }
            )*
        }
    }

    macro_rules! test_memory {
        ($(fn $name_test:ident() => $test_fn:ident ;)*) => {
            $(
                #[test]
                fn $name_test() {
                    let data = include_bytes!("../assets/lenna.png");
                    let image = ::image::$test_fn(&data[..], 3);
                    assert!(image.is_ok());
                    let image = image.unwrap();
                    assert_eq!(512, image.width());
                    assert_eq!(512, image.height());
                    assert_eq!(3, image.channels());
                    let image = ::image::$test_fn(vec![0; 4], 3);
                    assert!(image.is_err());
                }
            )*
        }
    }

    test_file! {
        fn test_load_from_file() => load;
        fn test_load_16_from_file() => load_16;
        fn test_loadf_from_file() => loadf;
    }

    test_memory! {
        fn test_load_from_memory() => load_from_memory;
        fn test_load_16_from_memory() => load_16_from_memory;
        fn test_loadf_from_memory() => loadf_from_memory;
    }

    #[test]
    fn test_ffi() {
        let mut x: raw::c_int = 0;
        let mut y: raw::c_int = 0;
        let mut channels: raw::c_int = 0;

        let lenna_file = CString::new("assets/lenna.png").unwrap();
        let data =
            unsafe { ffi::stbi_load(lenna_file.into_raw(), &mut x, &mut y, &mut channels, 3) };

        assert_eq!(512, x);
        assert_eq!(512, y);
        assert_eq!(3, channels);

        unsafe {
            ffi::stbi_image_free(data as _);
        }
    }
}
