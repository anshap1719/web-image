use image::DynamicImage;
use image::imageops::FilterType;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::image::WebImage;

#[wasm_bindgen]
impl WebImage {
    pub fn resize(self, max_width: u32, max_height: u32) -> Self {
        let dynamic_image: DynamicImage = self.into();

        (&dynamic_image.resize(max_width, max_height, FilterType::Nearest)).into()
    }

    pub fn resize_exact(self, width: u32, height: u32) -> Self {
        let dynamic_image: DynamicImage = self.into();

        (&dynamic_image.resize_exact(width, height, FilterType::Nearest)).into()
    }

    pub fn resize_to_fill(self, width: u32, height: u32) -> Self {
        let dynamic_image: DynamicImage = self.into();

        (&dynamic_image.resize_to_fill(width, height, FilterType::Nearest)).into()
    }
}