use reqwest::Client;
use serde_json::Value;

pub struct ProviderService {
    client: Client,
    api_key: String,
}

impl ProviderService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn list_intel_xeon(&self) -> Result<Value, reqwest::Error> {
        self.client
            .get("https://backend.datalix.de/v1/kvmserver/line/intelxeon")
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_amd_epyc(&self) -> Result<Value, reqwest::Error> {
        self.client
            .get("https://backend.datalix.de/v1/kvmserver/line/amdepyc")
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_intel_xeon_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("https://backend.datalix.de/v1/kvmserver/packet/{}", packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }
    pub async fn list_amd_epyc_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("https://backend.datalix.de/v1/kvmserver/packet/{}", packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }
    pub async fn list_intel_xeon_os_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("https://backend.datalix.de/v1/kvmserver/packet/{}/os", packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_amd_epyc_os_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("https://backend.datalix.de/v1/kvmserver/packet/{}/os", packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

}



