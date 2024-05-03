use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat};
use wasm_bindgen::{Clamped, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, ColorSpaceConversion, ImageBitmap, ImageBitmapOptions, ImageData, window};
use crate::util::blob_into_bytes;

#[derive(Clone)]
#[wasm_bindgen]
pub struct WebImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl From<&DynamicImage> for WebImage {
    fn from(image: &DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let raw_pixels = image.to_rgba8().to_vec();

        Self {
            raw_pixels,
            width,
            height,
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
        }
    }

    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        let img = image::load_from_memory_with_format(bytes, ImageFormat::Jpeg).unwrap();
        let raw_pixels = img.to_rgba8().to_vec();

        Self {
            raw_pixels,
            width: img.width(),
            height: img.height(),
        }
    }

    pub async fn from_blob(blob: Blob) -> Self {
        let bytes = blob_into_bytes(blob).await;
        Self::from_byte_slice(&bytes)
    }

    pub fn into_image_data(mut self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.raw_pixels),
            self.width,
            self.height,
        )
        .unwrap()
    }

    pub async fn into_image_bitmap(self) -> Result<ImageBitmap, JsValue> {
        let image_data = self.into_image_data();

        let mut options = ImageBitmapOptions::new();
        options.color_space_conversion(ColorSpaceConversion::Default);

        let future: JsFuture = window().unwrap().create_image_bitmap_with_image_data_and_image_bitmap_options(&image_data, &options)?.into();
        let bitmap = future.await?;

        Ok(bitmap.into())
    }

    pub fn raw_pixels(self) -> Vec<u8> {
        self.raw_pixels
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}