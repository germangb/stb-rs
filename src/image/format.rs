use super::ffi;

use std::os::raw;

pub trait PixelFormat {
    type Item: Copy;

    fn size() -> usize;

    unsafe fn load_from_memory(
        buffer: *const ffi::stbi_uc,
        len: raw::c_int,
        x: &mut raw::c_int,
        y: &mut raw::c_int,
        c: &mut raw::c_int,
    ) -> *mut Self;
}

macro_rules! impl_formats {
    (
        $(
            $(#[$meta_enum:meta])*
            $enum:ident {
                size => $size:expr,
                item => $item:ty,
                ffi => $ffi:path,
            }
        )+
    ) => {
        $(
            $(#[$meta_enum])*
            pub enum $enum {}
            impl PixelFormat for $enum {
                type Item = $item;
                #[inline]
                unsafe fn load_from_memory(b: *const ffi::stbi_uc,
                                           l: raw::c_int,
                                           x: &mut raw::c_int,
                                           y: &mut raw::c_int,
                                           c: &mut raw::c_int) -> *mut Self {
                    $ffi(b, l, x, y, c, Self::size() as _) as _
                }
                #[inline]
                fn size() -> usize {
                    $size
                }
            }
        )+
    }
}

impl_formats! {
    R8 {
        size => 1,
        item => (u8),
        ffi => ffi::stbi_load_from_memory,
    }

    Rg8 {
        size => 2,
        item => (u8, u8),
        ffi => ffi::stbi_load_from_memory,
    }

    Rgb8 {
        size => 3,
        item => (u8, u8, u8),
        ffi => ffi::stbi_load_from_memory,
    }

    Rgba8 {
        size => 4,
        item => (u8, u8, u8, u8),
        ffi => ffi::stbi_load_from_memory,
    }

    // 16bit

    R16 {
        size => 1,
        item => (u16),
        ffi => ffi::stbi_load_16_from_memory,
    }

    Rg16 {
        size => 2,
        item => (u16, u16),
        ffi => ffi::stbi_load_16_from_memory,
    }

    Rgb16 {
        size => 3,
        item => (u16, u16, u16),
        ffi => ffi::stbi_load_16_from_memory,
    }

    Rgba16 {
        size => 4,
        item => (u16, u16, u16, u16),
        ffi => ffi::stbi_load_16_from_memory,
    }

    // float32

    R32f {
        size => 1,
        item => (f32),
        ffi => ffi::stbi_loadf_from_memory,
    }

    Rg32f {
        size => 2,
        item => (f32, f32),
        ffi => ffi::stbi_loadf_from_memory,
    }

    Rgb32f {
        size => 3,
        item => (f32, f32, f32),
        ffi => ffi::stbi_loadf_from_memory,
    }

    Rgba32f {
        size => 4,
        item => (f32, f32, f32, f32),
        ffi => ffi::stbi_loadf_from_memory,
    }
}
