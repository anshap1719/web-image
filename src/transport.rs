#![cfg(feature = "bincode")]

use crate::image::WebImage;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct WebImageList(Vec<WebImage>);

#[wasm_bindgen]
impl WebImageList {
    pub fn images(self) -> Vec<WebImage> {
        self.0
    }

    pub fn set_images(&mut self, images: Vec<WebImage>) {
        self.0 = images;
    }

    pub fn encode(self) -> Uint8Array {
        let encoded: Vec<u8> = self.try_into().unwrap();
        let buffer = Uint8Array::new_with_length(encoded.len() as u32);
        buffer.copy_from(&encoded);

        buffer
    }

    pub fn decode_from(buffer: Uint8Array) -> Result<WebImageList, JsValue> {
        let bytes = buffer.to_vec();
        let decoded: WebImageList = bytes.try_into().map_err(|error: bincode::Error| {
            JsValue::from(js_sys::Error::new(&format!("{error:?}")))
        })?;

        Ok(decoded)
    }
}

impl TryFrom<Vec<u8>> for WebImageList {
    type Error = bincode::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bincode::deserialize(&value)
    }
}

impl TryFrom<WebImageList> for Vec<u8> {
    type Error = bincode::Error;

    fn try_from(value: WebImageList) -> Result<Self, Self::Error> {
        bincode::serialize(&value)
    }
}
