use crate::instance_utils::{parse_instances, RBXInstance};
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize)]
pub struct AssetLocation {
    pub location: String,
}

#[derive(Deserialize)]
pub struct AssetResponse {
    pub locations: Vec<AssetLocation>,
}

pub async fn get_asset_download_url(asset_id: &str) -> Result<String, String> {
    let url = format!("https://assetdelivery.roblox.com/v2/assetId/{}", asset_id);
    let response = reqwest::get(url).await.unwrap();

    if !response.status().is_success() {
        return Err(format!("Request for {} failed", asset_id));
    }

    let asset = response.json::<AssetResponse>().await.expect("request response should have valid json");

    // TODO: add support to multiple asset locations
    Ok(asset.locations.into_iter().nth(0).unwrap().location)
}

pub async fn download_asset(asset_id: &str) -> RBXInstance {
    let url = get_asset_download_url(asset_id).await.unwrap();
    let response = reqwest::get(url).await.unwrap();

    let bytes = response.bytes().await.unwrap();
    let cursor = Cursor::new(bytes);

    let dom = rbx_binary::from_reader(cursor).unwrap();

    parse_instances(dom)
}