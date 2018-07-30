use super::ffi;

use std::os::raw;

pub trait PixelFormat<D> {
    type Item: Copy;

    fn size() -> usize;
}

pub trait PixelData {
    unsafe fn load_from_memory(
        buffer: *const ffi::stbi_uc,
        len: raw::c_int,
        x: &mut raw::c_int,
        y: &mut raw::c_int,
        c: &mut raw::c_int,
        desired_c: raw::c_int,
    ) -> *mut ();
}

macro_rules! impl_data {
    ( $($type:ty => $ffi:path),+ ) => {
        $(
            impl PixelData for $type {
                unsafe fn load_from_memory(
                    buffer: *const ffi::stbi_uc,
                    len: raw::c_int,
                    x: &mut raw::c_int,
                    y: &mut raw::c_int,
                    c: &mut raw::c_int,
                    desired_c: raw::c_int,
                ) -> *mut () {
                    $ffi(buffer, len, x, y, c, desired_c) as _
                }
            }
        )+
    }
}

impl_data! {
    u8 => ffi::stbi_load_from_memory,
    u16 => ffi::stbi_load_16_from_memory,
    f32 => ffi::stbi_loadf_from_memory
}

macro_rules! impl_formats {
    (
        $(
            $(#[$meta_enum:meta])*
            $enum:ident {
                size => $size:expr,
                item<$T:ident> => $item:ty
            }
        ),+
    ) => {
        $(
            $(#[$meta_enum])*
            pub enum $enum {}
            impl<$T: Copy> PixelFormat<$T> for $enum {
                type Item = $item;
                #[inline]
                fn size() -> usize {
                    $size
                }
            }
        )+
    }
}

impl_formats! {
    R {
        size => 1,
        item<T> => (T)
    },
    Rg {
        size => 2,
        item<T> => (T, T)
    },
    Rgb {
        size => 3,
        item<T> => (T, T, T)
    },
    Rgba {
        size => 4,
        item<T> => (T, T, T, T)
    }
}
