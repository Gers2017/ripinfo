use anyhow::anyhow;
use flate2::bufread::GzDecoder;
use reqwest::Client;
use std::io::prelude::*;

use crate::{
    config::RipInfoConfig,
    ip_response::{IpData, IpResponseBusiness},
};

#[derive(Debug)]
pub struct RipInfo {
    pub client: Client,
}

impl RipInfo {
    pub fn new(user_agent: UserAgent) -> anyhow::Result<Self> {
        let client = Client::builder()
            .user_agent(user_agent.to_string())
            .build()?;

        Ok(Self { client })
    }

    pub async fn fetch_api_data(&self, ip: &str, config: &RipInfoConfig) -> anyhow::Result<IpData> {
        if config.use_token && config.token.is_none() {
            return Err(anyhow!("Missing token inside ripinfo_config.json"));
        }

        let data: IpData = if config.use_token {
            let token = config.token.clone().unwrap();
            let url = format!("https://ipinfo.io/{}/json?token={}", ip, &token);

            let response = self
                .client
                .get(url)
                .header("Referer", "https://ipinfo.io/")
                .header("Accept", "application/json")
                .send()
                .await?;
            let text = response.text().await?;
            serde_json::from_str::<IpData>(&text)?
        } else {
            let url = format!("https://ipinfo.io/widget/demo/{}", &ip);
            let cookie_val = "flash=; jwt-express=eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjo4NzYwOTQsImVtYWlsIjoiR2VyczIwMTdAb3V0bG9vay5jb20iLCJjcmVhdGVkIjoiYSBmZXcgc2Vjb25kcyBhZ28oMjAyMy0wMy0wNFQxNzoxNzozOC4zMzJaKSIsInN0cmlwZV9pZCI6bnVsbCwiaWF0IjoxNjc3OTUwMjU4LCJleHAiOjE2ODA1NDIyNTh9.AXU025TfoTZ3WGHWsV_HSiR_Gvc7H9Q2Gqt0jKuDamxWp8PsFLgrcWPCxfKHvIReTQaDnUD4135NBX2rYBout_m7AHCBTzbeaPWpsDFHbO19tCzOBdU0tYxLbtZLdWxEUHUHMctvUXlktpsm4aqUAdgL75_6otp5a95iQ8HD21MwIT3e";

            let response = self
                .client
                .get(url)
                .header("Cookie", cookie_val)
                .header("Referer", "https://ipinfo.io/")
                .header("Accept", "application/json")
                .header("Accept-Encoding", "gzip, deflate, br")
                .header("Sec-Fetch-Mode", "cors")
                .send()
                .await?;

            // decompress gzip data
            let mut buffer = String::new();
            let bytes = response.bytes().await?;
            let mut decoder = GzDecoder::new(&bytes[..]);
            let text = decoder.read_to_string(&mut buffer)?.to_string();

            let ip_data = serde_json::from_str::<IpResponseBusiness>(&text).map(|it| it.data)?;
            ip_data
        };

        Ok(data)
    }
}

#[derive(Debug)]
pub enum UserAgent {
    Chrome,
    Firefox,
    Edge,
}

impl std::fmt::Display for UserAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserAgent::Chrome => String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36"),
            UserAgent::Firefox => String::from("Mozilla/5.0 (Windows NT 10.0; rv:100.0) Gecko/20100101 Firefox/100.0"),
            UserAgent::Edge => String::from("Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36 Edg/109.0.1474.0"),
        };

        write!(f, "{}", s)
    }
}

impl Default for UserAgent {
    fn default() -> Self {
        UserAgent::Chrome
    }
}
