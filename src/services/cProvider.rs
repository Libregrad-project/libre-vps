use reqwest::Client;
use serde_json::Value;

pub struct CustomProviderService {
    client: Client,
    cc_api_key: String,
    cc_base_url: String,
    cc_control_server_ip: String,
}

impl CustomProviderService {
    pub fn new(
        cc_base_url: String,
        cc_api_key: String,
    ) -> Self {
        Self {
            client: Client::new(),
            cc_base_url,
            cc_api_key,
            cc_control_server_ip,
        }
    }
  // TODO: Implement our own backend, no provider except ourselfs.

}



