use reqwest::Client;
use serde_json::Value;

pub struct CustomProviderService {
    client: Client,
    api_key: String,
}

impl CustomProviderService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

  // TODO: Implement our own backend, no provider except ourselfs.
}



