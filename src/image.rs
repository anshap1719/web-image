use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat};
use wasm_bindgen::{Clamped, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, ColorSpaceConversion, ImageBitmap, ImageBitmapOptions, ImageData, window};
use crate::format::ImageFormatInt;
use crate::util::blob_into_bytes;

#[wasm_bindgen]
pub struct WebImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
    format: ImageFormat,
}

impl From<&DynamicImage> for WebImage {
    fn from(image: &DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let raw_pixels = image.to_rgba8().to_vec();

        Self {
            raw_pixels,
            width,
            height,
            format: ImageFormat::Jpeg,
        }
    }
}

impl Into<DynamicImage> for WebImage {
    fn into(self) -> DynamicImage {
        let buffer = ImageBuffer::from_raw(self.width, self.height, self.raw_pixels).unwrap();
        DynamicImage::ImageRgba8(buffer)
    }
}

#[wasm_bindgen]
impl WebImage {
    pub fn new(raw_pixels: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            raw_pixels,
            width,
            height,
            format: ImageFormat::Jpeg,
        }
    }

    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        let img = image::load_from_memory_with_format(bytes, ImageFormat::Jpeg).unwrap();
        let raw_pixels = img.to_rgba8().to_vec();

        Self {
            raw_pixels,
            width: img.width(),
            height: img.height(),
            format: ImageFormat::Jpeg,
        }
    }

    pub fn from_byte_slice_with_format(bytes: &[u8], format: ImageFormatInt) -> Self {
        let format = ImageFormat::from(format);
        let img = image::load_from_memory_with_format(bytes, format).unwrap();
        let raw_pixels = img.to_rgba8().to_vec();

        Self {
            raw_pixels,
            format,
            width: img.width(),
            height: img.height(),
        }
    }

    pub async fn from_blob(blob: Blob) -> Self {
        let bytes = blob_into_bytes(blob).await;
        Self::from_byte_slice(&bytes)
    }

    pub async fn from_blob_with_format(blob: Blob, format: ImageFormatInt) -> Self {
        let bytes = blob_into_bytes(blob).await;
        Self::from_byte_slice_with_format(&bytes, format)
    }

    pub fn into_image_data(self) -> ImageData {
        let dynamic_image: DynamicImage = self.into();
        let mut raw_pixels = dynamic_image.to_rgba8().to_vec();

        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut raw_pixels),
            dynamic_image.width(),
            dynamic_image.height(),
        )
        .unwrap()
    }

    pub fn get_format(&self) -> ImageFormatInt {
        self.format.into()
    }

    pub async fn into_image_bitmap(self) -> Result<ImageBitmap, JsValue> {
        let image_data = self.into_image_data();

        let mut options = ImageBitmapOptions::new();
        options.color_space_conversion(ColorSpaceConversion::Default);

        let future: JsFuture = window().unwrap().create_image_bitmap_with_image_data_and_image_bitmap_options(&image_data, &options)?.into();
        let bitmap = future.await?;

        Ok(bitmap.into())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}