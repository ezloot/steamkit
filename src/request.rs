use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

pub struct Config<T: Serialize + Sized> {
    pub iface: &'static str,
    pub method: &'static str,
    pub version: &'static str,
    pub query: Option<T>,
}

macro_rules! request {
    ($httpMethod:ident) => {
        request!($httpMethod, |_headers| {});
    };
    ($httpMethod:ident, $headers:expr) => {
        pub async fn $httpMethod<T: Serialize + Sized>(
            Config {
                iface,
                method,
                version,
                query,
            }: &Config<T>,
        ) -> Result<(), Error> {
            let mut headers = HeaderMap::new();
            headers.insert("Accept", HeaderValue::from_static("text/html,*/*;q=0.9"));
            headers.insert(
                "Accept-Encoding",
                HeaderValue::from_static("gzip,identity,*;q=0"),
            );
            headers.insert(
                "Accept-Charset",
                HeaderValue::from_static("ISO-8859-1,utf-8,*;q=0.7"),
            );

            $headers(&mut headers);

            let client = reqwest::Client::builder()
                .user_agent("Valve/Steam HTTP Client 1.0")
                .default_headers(headers)
                // .no_gzip()
                .build()?;

            let path = format!(
                "https://api.steampowered.com/{iface}/{method}/v{version}/",
                iface = iface,
                method = method,
                version = version,
            );

            let mut req = client.$httpMethod(path);
            if let Some(query) = query {
                req = req.query(query).query(&[("format", "vdf")]);
            }

            let res = req.send().await?;
            std::fs::write("data.vdf", res.bytes().await?).unwrap();

            Ok(())
        }
    };
}

request!(get);
request!(post, |headers: &mut HeaderMap| {
    headers.insert(
        "application/x-www-form-urlencoded",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
});
