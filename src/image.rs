use crate::error::WebImageError;
use crate::util::blob_into_bytes;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{Clamped, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Blob, ColorSpaceConversion, ImageBitmap, ImageBitmapOptions, ImageData};

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

impl TryFrom<WebImage> for DynamicImage {
    type Error = WebImageError;

    fn try_from(value: WebImage) -> Result<Self, Self::Error> {
        let buffer = ImageBuffer::from_raw(value.width, value.height, value.raw_pixels)
            .ok_or(WebImageError::ConversionError)?;

        Ok(DynamicImage::ImageRgba8(buffer))
    }
}

type WebImageResult = Result<WebImage, WebImageError>;

#[wasm_bindgen]
impl WebImage {
    #[must_use]
    pub fn new(raw_pixels: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            raw_pixels,
            width,
            height,
        }
    }

    /// Try to create a new WebImage from a byte slice
    ///
    /// returns: Result<WebImage, WebImageError>
    ///
    /// # Errors
    ///
    /// Errors if there's an error while decoding the image from given data
    pub fn try_from_byte_slice(bytes: &[u8]) -> WebImageResult {
        let img = image::load_from_memory_with_format(bytes, ImageFormat::Jpeg)?;
        let raw_pixels = img.to_rgba8().to_vec();

        Ok(Self {
            raw_pixels,
            width: img.width(),
            height: img.height(),
        })
    }

    /// Try to create a new WebImage from a blob
    ///
    /// returns: Result<WebImage, WebImageError>
    ///
    /// # Errors
    ///
    /// Errors if there's an error while decoding the image from given data
    pub async fn try_from_blob(blob: Blob) -> WebImageResult {
        let bytes = blob_into_bytes(blob)
            .await
            .map_err(Into::<WebImageError>::into)?;

        Self::try_from_byte_slice(&bytes)
    }

    /// Try to convert a WebImage to `ImageDate` that can be used with
    /// `ImageData` web APIs
    ///
    /// returns: Result<ImageData, JsValue>
    ///
    /// # Errors
    ///
    /// Errors if the web API throws an exception
    pub fn try_into_image_data(self) -> Result<ImageData, JsValue> {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.raw_pixels),
            self.width,
            self.height,
        )
    }

    /// Try to convert a WebImage to `ImageBitmap` that can be used with
    /// canvas APIs
    ///
    /// returns: Result<ImageBitmap, JsValue>
    ///
    /// # Errors
    ///
    /// Errors if the web API throws an exception
    pub async fn into_image_bitmap(self) -> Result<ImageBitmap, JsValue> {
        let image_data = self.try_into_image_data()?;

        let mut options = ImageBitmapOptions::new();
        options.color_space_conversion(ColorSpaceConversion::Default);

        let future: JsFuture = window()
            .ok_or(WebImageError::DomError)
            .map_err(Into::<JsValue>::into)?
            .create_image_bitmap_with_image_data_and_image_bitmap_options(&image_data, &options)?
            .into();

        let bitmap = future.await?;

        Ok(bitmap.into())
    }

    #[must_use]
    pub fn raw_pixels(self) -> Vec<u8> {
        self.raw_pixels
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }
}
