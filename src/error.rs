use image::ImageError;
use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum WebImageError {
    #[error("failed to convert web image to dynamic image: container is not large enough")]
    ConversionError,
    #[error("Failed to create web image")]
    ImageError(#[from] ImageError),
    #[error("A DOM error occurred")]
    DomError,
    #[error("A JS error occurred")]
    JsError { inner: JsValue },
}

impl From<WebImageError> for JsValue {
    fn from(value: WebImageError) -> Self {
        JsValue::from_str(value.to_string().as_str())
    }
}

impl From<JsValue> for WebImageError {
    fn from(value: JsValue) -> Self {
        WebImageError::JsError { inner: value }
    }
}
