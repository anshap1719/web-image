use crate::error::WebImageError;
use crate::image::WebImage;
use image::imageops::FilterType;
use image::DynamicImage;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
impl WebImage {
    /// Resize this image while preserving aspect ratio. The image is scaled to the maximum
    /// possible size that fits within the bounds specified by max_width and max_height.
    ///
    /// returns: Result<WebImage, WebImageError>
    ///
    /// # Errors
    ///
    /// Errors if conversion to DynamicImage fails
    pub fn resize(self, max_width: u32, max_height: u32) -> Result<WebImage, WebImageError> {
        let dynamic_image: DynamicImage = self.try_into()?;

        Ok((&dynamic_image.resize(max_width, max_height, FilterType::Nearest)).into())
    }

    /// Resize this image while ignoring aspect ratio. The width and height provided become the
    /// new dimensions for the image
    ///
    /// returns: Result<WebImage, WebImageError>
    ///
    /// # Errors
    ///
    /// Errors if conversion to DynamicImage fails
    pub fn resize_exact(self, width: u32, height: u32) -> Result<WebImage, WebImageError> {
        let dynamic_image: DynamicImage = self.try_into()?;

        Ok((&dynamic_image.resize_exact(width, height, FilterType::Nearest)).into())
    }

    /// Resize this image while preserving aspect ratio. The image is scaled to the maximum
    /// possible size that fits within the larger (relative to aspect ratio) of the bounds
    /// specified by width and height, then cropped to fit within the other bound.
    ///
    /// returns: Result<WebImage, WebImageError>
    ///
    /// # Errors
    ///
    /// Errors if conversion to DynamicImage fails
    pub fn resize_to_fill(self, width: u32, height: u32) -> Result<WebImage, WebImageError> {
        let dynamic_image: DynamicImage = self.try_into()?;

        Ok((&dynamic_image.resize_to_fill(width, height, FilterType::Nearest)).into())
    }
}
