use anyhow::anyhow;

use ripinfo::{
    ip_response::{IpData, IpResponseBusiness},
    parser, print_usage, RipInfo, UserAgent,
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

pub fn load_cache() -> anyhow::Result<RipInfoCache> {
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

pub fn update_cache(cache: &RipInfoCache) -> anyhow::Result<()> {
    let project_dir = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or(anyhow!("error on project dirs"))?;

    let mut cache_file = project_dir.config_dir().to_path_buf();
    cache_file.push(CACHE_FILE);

    let json_str = serde_json::to_string(cache)?;
    fs::write(&cache_file, json_str)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = args().skip(1).map(|it| it.trim().to_lowercase()).collect();

    if args.is_empty() {
        print_usage();
        return;
    }

    match args.first().unwrap().as_ref() {
        "--help" => {
            print_usage();
        }
        arg => {
            if let Err(err) = handle_ip_command(arg).await {
                eprintln!("{}", err);
            }
        }
    };
}

async fn handle_ip_command(arg: &str) -> anyhow::Result<()> {
    let mut cache = load_cache()?;
    let (_, ip) =
        parser::parse_ip_address(&arg).map_err(|err| anyhow!("Invalid ip shape\n{}", err))?;

    // check cache
    let ip_data: IpData = if let Some(cached_ip_data) = cache.get(&ip.text) {
        cached_ip_data.clone()
    } else {
        // else use ip to fetch data
        fetch_ip_data(&ip.text).await?
    };

    // print the data
    let json_to_print = serde_json::to_string(&ip_data)?;
    println!("{}", &json_to_print.to_colored_json_auto()?);

    // save the data on the cache file
    cache.insert(ip_data.ip.clone(), ip_data);
    update_cache(&cache)?;

    Ok(())
}

async fn fetch_ip_data(ip: &str) -> anyhow::Result<IpData> {
    let url = format!("https://ipinfo.io/widget/demo/{}", &ip);

    // ignore
    let cookie_val = "flash=; jwt-express=eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjo4NzYwOTQsImVtYWlsIjoiR2VyczIwMTdAb3V0bG9vay5jb20iLCJjcmVhdGVkIjoiYSBmZXcgc2Vjb25kcyBhZ28oMjAyMy0wMy0wNFQxNzoxNzozOC4zMzJaKSIsInN0cmlwZV9pZCI6bnVsbCwiaWF0IjoxNjc3OTUwMjU4LCJleHAiOjE2ODA1NDIyNTh9.AXU025TfoTZ3WGHWsV_HSiR_Gvc7H9Q2Gqt0jKuDamxWp8PsFLgrcWPCxfKHvIReTQaDnUD4135NBX2rYBout_m7AHCBTzbeaPWpsDFHbO19tCzOBdU0tYxLbtZLdWxEUHUHMctvUXlktpsm4aqUAdgL75_6otp5a95iQ8HD21MwIT3e";

    let ripinfo = RipInfo::new(&url, cookie_val, ripinfo::UserAgent::Chrome)?;
    let data = ripinfo.fetch_api_data().await?;
    let api_response = serde_json::from_str::<IpResponseBusiness>(&data)
        .map(|it| it.data)
        .map_err(|err| anyhow!("{}", err.to_string()));

    api_response
}
