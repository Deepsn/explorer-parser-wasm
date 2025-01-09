mod utils;
mod asset_utils;

use crate::asset_utils::download_asset;
pub use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn parse_asset(asset_id: &str) -> String {
    let response = download_asset(asset_id);

    response.await.unwrap().locations.into_iter().nth(0).unwrap().location
}