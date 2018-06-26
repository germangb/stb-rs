use std::{ffi::{CStr, CString},
          //os::raw,
          path::Path};

use std::io::Read;

use {Result, Error};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/stb_image.rs"));
}

#[derive(Debug)]
pub struct Image<S> {
    width: usize,
    height: usize,
    channels: usize,
    data: *mut S,
}

unsafe impl<S> Send for Image<S> {
}

impl<S> Drop for Image<S> {
    fn drop(&mut self) {
        unsafe { ffi::stbi_image_free(self.data as _) }
    }
}

impl<S> Image<S> {
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

macro_rules! from_memory {
    ( $(pub fn $fn_name:ident ( ffi::$stb_fn:ident ) => Image<$out_type:ty> ; )*) => {
        $(
            pub fn $fn_name<M>(memory: M, desired_channels: usize) -> Result<Image<$out_type>>
            where
                M: AsRef<[u8]>,
            {
                let data = memory.as_ref();
                let mut width = 0;
                let mut height = 0;
                let mut channels = 0;

                unsafe {
                    let image_data = ffi::$stb_fn(data.as_ptr(),
                                                  data.len() as _,
                                                  &mut width,
                                                  &mut height,
                                                  &mut channels,
                                                  desired_channels as _);
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
        )*
    }
}

macro_rules! from_reader {
    ( $(pub fn $fn_name:ident ( $memory_fn:ident ) => Image<$out_type:tt> ; )*) => {
        $(
            pub fn $fn_name<R: Read>(mut reader: R, desired_channels: usize) -> Result<Image<$out_type>> {
                let mut data = Vec::new();
                match reader.read_to_end(&mut data) {
                    Err(e) => Err(Error::Io(e)),
                    Ok(_) => $memory_fn(data, desired_channels),
                }
            }
        )*
    }
}

macro_rules! from_file {
    ( $(pub fn $fn_name:ident ( ffi::$stb_fn:ident ) => Image<$out_type:tt> ; )*) => {
        $(
            pub fn $fn_name<P>(path: P, desired_channels: usize) -> Result<Image<$out_type>>
            where
                P: AsRef<Path>,
            {
                //TODO This shouldn't panic
                let path = path.as_ref();

                let mut width = 0;
                let mut height = 0;
                let mut channels = 0;

                unsafe {
                    let image_data = ffi::$stb_fn(CString::new(path.to_str().unwrap()).unwrap().into_raw(),
                                                  &mut width,
                                                  &mut height,
                                                  &mut channels,
                                                  desired_channels as _);
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
        )*
    }
}

from_file! {
    pub fn load(ffi::stbi_load) => Image<u8>;
    pub fn loadf(ffi::stbi_loadf) => Image<f32>;
    pub fn load_16(ffi::stbi_load_16) => Image<u16>;
}

from_memory! {
    pub fn load_from_memory(ffi::stbi_load_from_memory) => Image<u8>;
    pub fn loadf_from_memory(ffi::stbi_loadf_from_memory) => Image<f32>;
    pub fn load_16_from_memory(ffi::stbi_load_16_from_memory) => Image<u16>;
}

from_reader! {
    pub fn load_from_reader(load_from_memory) => Image<u8>;
    pub fn loadf_from_reader(loadf_from_memory) => Image<f32>;
    pub fn load_16_from_reader(load_16_from_memory) => Image<u16>;
}

#[cfg(test)]
mod tests {
    use image::{Image, ffi};

    use std::os::raw;
    use std::ffi::CString;

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
