use std::ffi::CStr;
use std::fs::File;
use std::path::Path;

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

    unsafe fn free(ptr: *const Self) {
        ffi::stbi_image_free(ptr as _)
    }
}

macro_rules! impl_traits {
    ($($type:ty => $mem:ident,)+) => {
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
    u8 => stbi_load_from_memory,
    u16 => stbi_load_16_from_memory,
    f32 => stbi_loadf_from_memory,
}

#[derive(Debug)]
pub struct Image<S: Data> {
    width: usize,
    height: usize,
    channels: usize,
    data: *mut S,
}

unsafe impl<S: Data> Send for Image<S> {}

impl<S: Data> Drop for Image<S> {
    fn drop(&mut self) {
        unsafe { S::free(self.data) }
    }
}

impl<S: Data> ::std::ops::Deref for Image<S> {
    type Target = [S];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let len = self.width * self.height * self.channels;
            ::std::slice::from_raw_parts(self.data, len)
        }
    }
}

impl<S: Data> ::std::ops::DerefMut for Image<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let len = self.width * self.height * self.channels;
            ::std::slice::from_raw_parts_mut(self.data, len)
        }
    }
}

impl<S: Data> Image<S> {
    pub fn from_file<P: AsRef<Path>>(path: P, desired_channels: usize) -> Result<Self> {
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
    use image::Image;

    macro_rules! test_type {
        ($($type:ty,)+) => {
            #[test]
            fn test() {
                $(
                    let im = Image::<$type>::from_file("assets/lenna.png", 3).unwrap();
                    assert_eq!(512, im.width());
                    assert_eq!(512, im.height());
                    assert_eq!(3, im.channels());
                    assert_eq!(512*512*3, im.len());
                    assert!(Image::<$type>::from_file(".", 3).is_err());
                )+
            }
        }
    }

    test_type! {
        f32, u16, u8,
    }
}
