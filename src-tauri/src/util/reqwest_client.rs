use std::sync::Arc;

use futures::lock::Mutex;
use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};

use crate::config::model::LauncherConfig;

pub type SharedClient = Arc<Mutex<Option<Client>>>;

pub static REQWEST_CLIENT: Lazy<SharedClient> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn create_reqwest_client(config: &LauncherConfig) -> Result<Client, reqwest::Error> {
    let mut builder = Client::builder();
    if config.enable_proxy {
        match &config.proxy.host {
            Some(host) if !host.trim().is_empty() => {
                let proxy_result = if config.proxy.enable_auth {
                    match (
                        config.proxy.username.as_ref(),
                        config.proxy.password.as_ref(),
                    ) {
                        (Some(username), Some(password)) => {
                            Proxy::all(host).and_then(|p| Ok(p.basic_auth(username, password)))
                        }
                        _ => Proxy::all(host),
                    }
                } else {
                    Proxy::all(host)
                };
                match proxy_result {
                    Ok(proxy) => {
                        builder = builder.proxy(proxy);
                    }
                    Err(e) => {
                        eprintln!(
                            "Proxy Config Error: host: '{}', auth: {}, username: '{}', error: {:?} | {}",
                            host,
                            config.proxy.enable_auth,
                            config.proxy.username.as_deref().unwrap_or("<none>"),
                            e,
                            e
                        );
                        // 继续使用无代理的 builder
                    }
                }
            }
            _ => {
                eprintln!(
                    "Proxy enabled but host is empty or None. Config: enable_proxy={}, proxy: {:?}",
                    config.enable_proxy, config.proxy
                );
            }
        }
    }
    builder.build()
}

pub async fn update_reqwest_client(config: &LauncherConfig) {
    match create_reqwest_client(config) {
        Ok(new_client) => {
            *REQWEST_CLIENT.lock().await = Some(new_client);
            println!("Client updated with new proxy settings");
        }
        Err(e) => eprintln!("Failed to update client: {}", e),
    }
}
