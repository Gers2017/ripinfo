pub mod ip_response;
pub mod parser;

use anyhow;
use flate2::bufread::GzDecoder;
use reqwest::Client;
use std::io::prelude::*;

#[derive(Debug)]
pub struct RipInfo {
    pub url: String,
    pub cookie: String,
    pub client: Client,
}

#[derive(Debug)]
pub enum UserAgent {
    Chrome,
    Firefox,
}

impl std::fmt::Display for UserAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserAgent::Chrome => String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36"),
            UserAgent::Firefox => String::from("Mozilla/5.0 (Windows NT 10.0; rv:100.0) Gecko/20100101 Firefox/100.0"),
        };

        write!(f, "{}", s)
    }
}

impl RipInfo {
    pub fn new(url: &str, cookie: &str, user_agent: UserAgent) -> anyhow::Result<Self> {
        let client = Client::builder()
            .user_agent(user_agent.to_string())
            .build()?;

        Ok(Self {
            url: url.to_string(),
            cookie: cookie.to_string(),
            client,
        })
    }

    pub async fn fetch_api_data(&self) -> anyhow::Result<String> {
        // make request with cookie and referer headers
        let response = self
            .client
            .get(&self.url)
            .header("Cookie", &self.cookie)
            .header("Referer", "https://ipinfo.io/")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Sec-Fetch-Mode", "cors")
            .send()
            .await?;

        dbg!(&response);

        // decompress gzip data
        let bytes = response.bytes().await?;
        let mut buffer = String::new();
        let mut decoder = GzDecoder::new(&bytes[..]);
        decoder.read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}

pub fn print_usage() {
    println!(
        "{}",
        r#"USAGE: ripinfo [IP] [OPTIONS]...
OPTIONS                 DESCRIPTION             VALUES
--help                  Prints this message
--user-agent=[NAME]     Sets the user-agent     chrome, firefox
"#
    )
}
