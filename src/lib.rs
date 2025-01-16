mod utils;
mod asset_utils;

use crate::asset_utils::download_asset;
pub use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn parse_asset(asset_id: &str) -> JsValue {
    let response = download_asset(asset_id);

    serde_wasm_bindgen::to_value(&response.await).unwrap()
}