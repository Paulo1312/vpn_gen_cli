use serde_derive::{Deserialize, Serialize};

use reqwest::Client;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Token {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigJson{
    pub amnz_ovc_config: AmneziaConfig,
    pub outline_config: OutlineConfig,
    pub user_name: String,
    pub wireguard_config: WireguardConfig,
    pub v_p_n_gen_config: String,
    pub proto0_config: Proto0
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Proto0 {
    pub access_key: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AmneziaConfig {
    pub file_content: String,
    pub file_name: String,
    pub tonnel_name: String
}


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutlineConfig {
    pub access_key: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WireguardConfig {
    pub file_content: String,
    pub file_name: String,
    pub tonnel_name: String
}

/// Every "brigade owner" has unique prefix in address. We get it using this function
pub async fn get_base_url() -> Result<String, Box<dyn std::error::Error>>{
    Ok(reqwest::get("http://vpn.works").await?.url().as_str().to_string())
}

/// Get token using post request to prefix.vpn.works/token
pub async fn get_token(client: &Client, base_url: &String) -> Result<Token, Box<dyn std::error::Error>>{
    let token = client.post(format!("{}/token",base_url)).send().await?.json::<Token>().await?;
    Ok(token)
}

/// Get configs for amnezia, outline, wireguard
pub async fn get_config(client: &Client) -> Result<ConfigJson, Box<dyn std::error::Error>>{
    let base_url = get_base_url().await?;
    let token = get_token(&client, &base_url).await?;

    Ok(client.post(format!("{}/user",base_url)).bearer_auth(token.token).send().await?.json::<ConfigJson>().await?)
}