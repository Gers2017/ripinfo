use ripinfo::{
    config::{load_cache, update_cache},
    ip_response::{IpData, IpResponseBusiness},
    parser::parse_ip_address,
    print_usage, RipInfo, UserAgent,
};

use anyhow::anyhow;
use std::env::args;
extern crate colored_json;
use colored_json::prelude::*;

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
        },
        arg => {
            let user_agent = args
                .get(1)
                .map(|s| match s.as_ref() {
                    "--firefox" => UserAgent::Firefox,
                    "--edge" => UserAgent::Edge,
                    _ => UserAgent::Chrome,
                })
                .unwrap_or_else(|| UserAgent::default());

            if let Err(err) = handle_ip_command(arg, user_agent).await {
                eprintln!("{}", err);
            }
        }
    };
}

async fn handle_ip_command(arg: &str, user_agent: UserAgent) -> anyhow::Result<()> {
    let mut cache = load_cache()?;
    let (_, ip) = parse_ip_address(&arg).map_err(|err| anyhow!("Invalid ip shape\n{}", err))?;
    // check cache
    let ip_data: IpData = if let Some(cached_ip_data) = cache.get(&ip.text) {
        cached_ip_data.clone()
    } else {
        // else use ip to fetch data
        fetch_ip_data(&ip.text, user_agent).await?
    };

    // print the data
    let json_to_print = serde_json::to_string(&ip_data)?;
    println!("{}", &json_to_print.to_colored_json_auto()?);

    // save the data on the cache file
    cache.insert(ip_data.ip.clone(), ip_data);
    update_cache(&cache)?;

    Ok(())
}

async fn fetch_ip_data(ip: &str, user_agent: UserAgent) -> anyhow::Result<IpData> {
    let url = format!("https://ipinfo.io/widget/demo/{}", &ip);
    // ignore
    let cookie_val = "flash=; jwt-express=eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjo4NzYwOTQsImVtYWlsIjoiR2VyczIwMTdAb3V0bG9vay5jb20iLCJjcmVhdGVkIjoiYSBmZXcgc2Vjb25kcyBhZ28oMjAyMy0wMy0wNFQxNzoxNzozOC4zMzJaKSIsInN0cmlwZV9pZCI6bnVsbCwiaWF0IjoxNjc3OTUwMjU4LCJleHAiOjE2ODA1NDIyNTh9.AXU025TfoTZ3WGHWsV_HSiR_Gvc7H9Q2Gqt0jKuDamxWp8PsFLgrcWPCxfKHvIReTQaDnUD4135NBX2rYBout_m7AHCBTzbeaPWpsDFHbO19tCzOBdU0tYxLbtZLdWxEUHUHMctvUXlktpsm4aqUAdgL75_6otp5a95iQ8HD21MwIT3e";

    let ripinfo = RipInfo::new(&url, cookie_val, user_agent)?;
    let data = ripinfo.fetch_api_data().await?;
    let api_response = serde_json::from_str::<IpResponseBusiness>(&data)
        .map(|it| it.data)
        .map_err(|err| anyhow!("{}", err.to_string()));

    api_response
}
