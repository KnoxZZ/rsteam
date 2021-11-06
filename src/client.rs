use crate::utils::{Error, ErrorKind, Result};
use hyper::client::HttpConnector;
use hyper::Body;
use hyper::Client as HyperClient;

use hyper_tls::HttpsConnector;

/// Client to make API requests easily.
///
/// Client can be constructed with or without an API key. Only subset
/// of APIs are available for the client.
pub struct SteamClient {
    pub(crate) client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    api_key: Option<String>,
}

impl SteamClient {
    /// Create a client with an API key.
    ///
    /// Client with API key can use all available APIs.
    pub fn with_api_key(key: &str) -> Self {
        let https_connector = HttpsConnector::new();

        SteamClient {
            client: HyperClient::builder().build::<_, Body>(https_connector),
            api_key: Some(key.to_owned()),
        }
    }

    /// Create a client without an API key.
    ///
    /// Client without an API key can only use a subset of the APIs.
    pub fn new() -> Self {
        let https_connector = HttpsConnector::new();

        SteamClient {
            client: HyperClient::builder().build::<_, Body>(https_connector),
            api_key: None,
        }
    }

    pub(crate) fn api_key(&self) -> Result<String> {
        match &self.api_key {
            Some(key) => Ok(key.to_owned()),
            None => Err(Error::new(ErrorKind::NoAPIKey)),
        }
    }
}
