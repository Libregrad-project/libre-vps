use reqwest::Client;
use serde_json::Value;

pub struct ProviderService {
    client: Client,
    d_base_url: String,
    d_api_key: String,
    c_base_url: String,
    c_api_key: String,
}

impl ProviderService {
    pub fn new(
        d_base_url: String,
        d_api_key: String,
        c_base_url: String,
        c_api_key: String,
    ) -> Self {
        Self {
            client: Client::new(),
            d_base_url,
            d_api_key,
            c_base_url,
            c_api_key,
        }
    }

    // -------------------
    // D-Provider
    // -------------------

    pub async fn list_intel_xeon(&self) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/line/intelxeon", self.d_base_url);
        self.client
            .get(&url)
            .bearer_auth(&self.d_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_amd_epyc(&self) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/line/amdepyc", self.d_base_url);
        self.client
            .get(&url)
            .bearer_auth(&self.d_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_intel_xeon_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}", self.d_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.d_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_amd_epyc_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}", self.d_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.d_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_intel_xeon_os_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}/os", self.d_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.d_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn list_amd_epyc_os_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}/os", self.d_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.d_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    // -------------------
    // C-Provider
    // -------------------

    pub async fn c_list_intel_xeon(&self) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/line/intelxeon", self.c_base_url);
        self.client
            .get(&url)
            .bearer_auth(&self.c_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn c_list_amd_epyc(&self) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/line/amdepyc", self.c_base_url);
        self.client
            .get(&url)
            .bearer_auth(&self.c_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn c_list_intel_xeon_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}", self.c_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.c_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn c_list_amd_epyc_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}", self.c_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.c_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn c_list_intel_xeon_os_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}/os", self.c_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.c_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn c_list_amd_epyc_os_information(&self, packetid: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/kvmserver/packet/{}/os", self.c_base_url, packetid);
        self.client
            .get(&url)
            .bearer_auth(&self.c_api_key)
            .send()
            .await?
            .json::<Value>()
            .await
    }
}
