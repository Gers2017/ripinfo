use anyhow::anyhow;

use ripinfo::{
    ip_response::{IpData, IpResponseBusiness},
    RipInfo,
};
use std::{env::args, fs};

extern crate colored_json;
use colored_json::prelude::*;

use std::collections::HashMap;
extern crate directories;
use directories::ProjectDirs;

type RipInfoCache = HashMap<String, IpData>;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "RipInfo";
const APPLICATION: &str = "ripinfo";
const CACHE_FILE: &str = "ripinfo.json";

pub fn load_config() -> anyhow::Result<RipInfoCache> {
    let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or(anyhow!("error on project dirs"))?;

    let config_dir = project_dirs.config_dir();

    if !&config_dir.exists() {
        fs::create_dir(config_dir)?;
    }
    let mut cache_file = config_dir.to_path_buf();
    cache_file.push(CACHE_FILE);

    match fs::read_to_string(&cache_file) {
        Ok(text) => Ok(serde_json::from_str::<RipInfoCache>(&text)?),
        Err(_e) => Ok(HashMap::new()),
    }
}

pub fn store_config(ripinfo_config: &RipInfoCache) -> anyhow::Result<()> {
    let project_dir = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or(anyhow!("error on project dirs"))?;

    let mut cache_file = project_dir.config_dir().to_path_buf();
    cache_file.push(CACHE_FILE);

    let json_str = serde_json::to_string(ripinfo_config)?;
    fs::write(&cache_file, json_str)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;
    let args: Vec<_> = args().skip(1).collect();

    let ip_data = match args.first() {
        Some(ip) => {
            let ip = ip.trim().to_lowercase();
            // assume the ip is correct, for now...

            // cache layer here...
            if let Some(cached_value) = config.get(&ip) {
                anyhow::Ok(cached_value.clone())
            } else {
                let url = format!("https://ipinfo.io/widget/demo/{}", &ip);
                // ignore
                let cookie_val = "flash=; jwt-express=eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjo4NzYwOTQsImVtYWlsIjoiR2VyczIwMTdAb3V0bG9vay5jb20iLCJjcmVhdGVkIjoiYSBmZXcgc2Vjb25kcyBhZ28oMjAyMy0wMy0wNFQxNzoxNzozOC4zMzJaKSIsInN0cmlwZV9pZCI6bnVsbCwiaWF0IjoxNjc3OTUwMjU4LCJleHAiOjE2ODA1NDIyNTh9.AXU025TfoTZ3WGHWsV_HSiR_Gvc7H9Q2Gqt0jKuDamxWp8PsFLgrcWPCxfKHvIReTQaDnUD4135NBX2rYBout_m7AHCBTzbeaPWpsDFHbO19tCzOBdU0tYxLbtZLdWxEUHUHMctvUXlktpsm4aqUAdgL75_6otp5a95iQ8HD21MwIT3e";

                let rp = RipInfo::new(&url, cookie_val, ripinfo::UserAgent::Chrome)?;
                let data = rp.fetch_api_data().await?;
                let ip_response = serde_json::from_str::<IpResponseBusiness>(&data)
                    .map_err(|err| anyhow!("{}", err.to_string()));

                ip_response.map(|x| x.data)
            }
        }
        None => Err(anyhow!("Missing ip argument")),
    }?;

    // print the data
    let json_to_print = serde_json::to_string(&ip_data)?;
    println!("{}", &json_to_print.to_colored_json_auto()?);

    // save the data on the cache file
    config.insert(ip_data.ip.clone(), ip_data);
    store_config(&config)?;
    Ok(())
}
