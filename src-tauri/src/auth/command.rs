use minecraft_msa_auth::{MinecraftAuthenticationResponse, MinecraftAuthorizationFlow, MinecraftTokenType};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::StandardDeviceAuthorizationResponse;
use oauth2::{AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, TokenResponse, TokenUrl};
use reqwest::Client;
use serde::{de, Deserialize, Serialize};
use tauri::ipc::Channel;

const DEVICE_CODE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const MSA_AUTHORIZE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const CLIENT_ID: &str = "";
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceAuthResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct UserCodeResult {
    verification_uri: String,
    user_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftAuthResponse {
    username: String,
    access_token: String,
    token_type: MinecraftTokenType,
    expires_in: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum LoginEvent {
    Started {
        code: String,
    },
    Finished {
        response: MinecraftAuthResponse
    },
}


#[tauri::command]
pub async fn get_device_code(on_event: Channel<LoginEvent>) -> Result<(), String> {
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        None,
        AuthUrl::new(MSA_AUTHORIZE_URL.to_string()).map_err(|e| e.to_string())?,
        Some(TokenUrl::new(MSA_TOKEN_URL.to_string()).map_err(|e| e.to_string())?),
        )
    .set_device_authorization_url(DeviceAuthorizationUrl::new(DEVICE_CODE_URL.to_string()).map_err(|e| e.to_string())?);

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code().map_err(|e| e.to_string())?
        .add_scope(Scope::new("XboxLive.signin offline_access".to_string()))
        .request_async(async_http_client)
        .await.map_err(|e| e.to_string())?;
 
    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        details.verification_uri().to_string(),
        details.user_code().secret().to_string()
    );
    // let result = UserCodeResult {
    //     verification_uri: details.verification_uri().to_string(),
    //     user_code: details.user_code().secret().to_string(),
    // };
    on_event.send(LoginEvent::Started {
        code: details.user_code().secret().to_string(),
    }).map_err(|e| e.to_string())?;


    // Wait for the user to complete the login process
    let token = client
        .exchange_device_access_token(&details)
        .request_async(async_http_client, tokio::time::sleep, None)
        .await
        .map_err(|e| e.to_string())?;
    let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
    let mc_token = mc_flow.exchange_microsoft_token(token.access_token().secret()).await.map_err(|e| e.to_string())?;
    let result = MinecraftAuthResponse {
        username: mc_token.username().to_string(),
        access_token: mc_token.access_token().clone().into_inner(),
        token_type: mc_token.token_type().clone(),
        expires_in: mc_token.expires_in(),
    };
    println!("Login successful for user: {}", result.username);
    on_event.send(LoginEvent::Finished {
        response: result.clone(),
    }).map_err(|e| e.to_string())?;
    Ok(())
    }


// #[tauri::command]
// pub async fn poll_login_status() -> Result<MinecraftAuthResponse, String> {
//     println!("Polling login status...");
//     let client = get_auth_client(CLIENT_ID.to_string()).await.map_err(|e| e.to_string())?;
//     let details: StandardDeviceAuthorizationResponse = client
//         .exchange_device_code().map_err(|e| e.to_string())?
//         .add_scope(Scope::new("XboxLive.signin offline_access".to_string()))
//         .request_async(async_http_client)
//         .await.map_err(|e| e.to_string())?;
//     let token = client
//         .exchange_device_access_token(&details)
//         .request_async(async_http_client, tokio::time::sleep, None)
//         .await
//         .map_err(|e| e.to_string())?;
//     let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
//     let mc_token = mc_flow.exchange_microsoft_token(token.access_token().secret()).await.map_err(|e| e.to_string())?;
//     let result = MinecraftAuthResponse {
//         username: mc_token.username().to_string(),
//         access_token: mc_token.access_token().clone().into_inner(),
//         token_type: mc_token.token_type().clone(),
//         expires_in: mc_token.expires_in(),
//     };
//     println!("Login successful for user: {}", result.username);
//     Ok(result).map_err(|e| e.to_string())
// }