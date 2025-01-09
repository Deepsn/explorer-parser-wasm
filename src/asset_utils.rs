use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssetLocation {
    pub location: String,
}

#[derive(Deserialize)]
pub struct AssetResponse {
    pub locations: Vec<AssetLocation>,
}

pub async fn download_asset(asset_id: &str) -> Result<AssetResponse, String> {
    let url = format!("https://assetdelivery.roblox.com/v2/assetId/{}", asset_id);
    let response = reqwest::get(url).await.unwrap();
    
    if !response.status().is_success() {
        return Err(format!("Request for {} failed", asset_id));
    }

    let json = response.json::<AssetResponse>().await.expect("request response should have valid json");
    
    Ok(json)
}