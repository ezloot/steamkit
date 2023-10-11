use reqwest::{
    header::{HeaderMap, ACCEPT, ACCEPT_CHARSET, ACCEPT_ENCODING, USER_AGENT},
    Client, StatusCode,
};
use serde::{Deserialize, Serialize};
use steamkit_kv::KeyValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebApiError {
    #[error("cannot build http client")]
    Client,
    #[error("request failed: {0}")]
    Request(StatusCode),
    #[error("request failed to send: `{0}`")]
    Send(#[from] reqwest::Error),
    #[error("invalid format: `{0}`")]
    Format(#[from] steamkit_kv::Error),
    #[error("unknown web api error")]
    Unknown,
}

pub struct WebApi {
    client: Client,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Servers {
    #[serde(alias = "serverlist")]
    pub tcp: Vec<String>,
    #[serde(alias = "serverlist_websockets")]
    pub websocket: Vec<String>,
}

impl WebApi {
    pub fn new() -> Result<Self, WebApiError> {
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, "text/html,*/*;q=0.9".parse().unwrap());
        headers.insert(ACCEPT_ENCODING, "gzip,identity,*;q=0".parse().unwrap());
        headers.insert(ACCEPT_CHARSET, "ISO-8859-1,utf-8,*;q=0.7".parse().unwrap());
        headers.insert(USER_AGENT, "Valve/Steam HTTP Client 1.0".parse().unwrap());

        Ok(Self {
            client: Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|_| WebApiError::Client)?,
        })
    }

    pub async fn get<S: Serialize>(
        &self,
        iface: &str,
        method: &str,
        version: u32,
        query: &S,
    ) -> Result<KeyValue, WebApiError> {
        let version = format!("{:04}", version);
        let url = format!("https://api.steampowered.com/{iface}/{method}/v{version}");
        let res = self
            .client
            .get(url)
            .query(&[("format", "vdf")])
            .query(&query)
            .send()
            .await?;

        if res.status().is_success() {
            let body = res.text().await?;
            let kv: KeyValue = KeyValue::parse(&body)?;
            Ok(kv)
        } else {
            Err(WebApiError::Request(res.status()))
        }
    }

    pub async fn get_servers(&self, cell_id: Option<u32>) -> Result<Servers, WebApiError> {
        let kv = self
            .get(
                "ISteamDirectory",
                "GetCMList",
                1,
                &[("cellid", cell_id.unwrap_or(1))],
            )
            .await?;

        let node = kv
            .root()
            .lookup("response")?
            .expect("failed to get response from `GetCMList` response")
            .to_owned();

        Ok(Servers::deserialize(node)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::WebApi;

    #[tokio::test]
    async fn get_servers() {
        let webapi = WebApi::new().unwrap();
        let servers = webapi.get_servers(None).await.unwrap();

        assert!(!servers.tcp.is_empty());
        assert!(!servers.websocket.is_empty());
    }
}
