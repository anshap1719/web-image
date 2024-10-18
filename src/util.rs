use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;

pub(crate) async fn blob_into_bytes(blob: Blob) -> Result<Vec<u8>, JsValue> {
    let array_buffer_promise: JsFuture = blob.array_buffer().into();

    let array_buffer: JsValue = array_buffer_promise.await?;

    Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
}
