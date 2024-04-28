use image::ImageFormat;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ImageFormatInt {
    Png = 1,
    Jpeg = 2,
    Gif = 3,
    WebP = 4,
    Pnm = 5,
    Tiff = 6,
    Tga = 7,
    Dds = 8,
    Bmp = 9,
    Ico = 10,
    Hdr = 11,
    OpenExr = 12,
    Farbfeld = 13,
    Avif = 14,
    Qoi = 15,
}

impl From<ImageFormatInt> for ImageFormat {
    fn from(value: ImageFormatInt) -> Self {
        match value {
            ImageFormatInt::Png => ImageFormat::Png,
            ImageFormatInt::Jpeg => ImageFormat::Jpeg,
            ImageFormatInt::Gif => ImageFormat::Gif,
            ImageFormatInt::WebP => ImageFormat::WebP,
            ImageFormatInt::Pnm => ImageFormat::Pnm,
            ImageFormatInt::Tiff => ImageFormat::Tiff,
            ImageFormatInt::Tga => ImageFormat::Tga,
            ImageFormatInt::Dds => ImageFormat::Dds,
            ImageFormatInt::Bmp => ImageFormat::Bmp,
            ImageFormatInt::Ico => ImageFormat::Ico,
            ImageFormatInt::Hdr => ImageFormat::Hdr,
            ImageFormatInt::OpenExr => ImageFormat::OpenExr,
            ImageFormatInt::Farbfeld => ImageFormat::Farbfeld,
            ImageFormatInt::Avif => ImageFormat::Avif,
            ImageFormatInt::Qoi => ImageFormat::Qoi,
        }
    }
}

impl From<ImageFormat> for ImageFormatInt {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Png => ImageFormatInt::Png,
            ImageFormat::Jpeg => ImageFormatInt::Jpeg,
            ImageFormat::Gif => ImageFormatInt::Gif,
            ImageFormat::WebP => ImageFormatInt::WebP,
            ImageFormat::Pnm => ImageFormatInt::Pnm,
            ImageFormat::Tiff => ImageFormatInt::Tiff,
            ImageFormat::Tga => ImageFormatInt::Tga,
            ImageFormat::Dds => ImageFormatInt::Dds,
            ImageFormat::Bmp => ImageFormatInt::Bmp,
            ImageFormat::Ico => ImageFormatInt::Ico,
            ImageFormat::Hdr => ImageFormatInt::Hdr,
            ImageFormat::OpenExr => ImageFormatInt::OpenExr,
            ImageFormat::Farbfeld => ImageFormatInt::Farbfeld,
            ImageFormat::Avif => ImageFormatInt::Avif,
            ImageFormat::Qoi => ImageFormatInt::Qoi,
            _ => unimplemented!("Unknown image format")
        }
    }
}