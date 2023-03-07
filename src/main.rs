use ripinfo::{
    config::{load_cache, update_cache, RipInfoConfig},
    ip_response::IpData,
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

    let config = RipInfoConfig::load_config();
    if let Err(err) = config {
        eprintln!("{}", err);
        return;
    }

    let mut config = config.unwrap();

    match args.first().unwrap().as_ref() {
        "--help" => {
            print_usage();
        }
        "--show-token" => {
            println!("{}", config);
        }
        "--show-use-token" => {
            println!("{}", config.use_token);
        }
        "--toggle-token" => {
            config.use_token = !config.use_token;
            if let Err(err) = RipInfoConfig::save_config(&config) {
                eprintln!("{}", err);
            }
            return;
        }
        arg => {
            let user_agent = args
                .get(1)
                .map(|s| match s.as_ref() {
                    "--firefox" => UserAgent::Firefox,
                    "--edge" => UserAgent::Edge,
                    _ => UserAgent::Chrome,
                })
                .unwrap_or_else(|| UserAgent::default());

            if let Err(err) = handle_ip_command(arg, user_agent, &config).await {
                eprintln!("{}", err);
            }
        }
    };
}

async fn handle_ip_command(
    arg: &str,
    user_agent: UserAgent,
    config: &RipInfoConfig,
) -> anyhow::Result<()> {
    let mut cache = load_cache()?;

    let (_, ip) = parse_ip_address(&arg).map_err(|err| anyhow!("Invalid ip shape\n{}", err))?;
    // check cache
    let ip_data: IpData = if let Some(cached_ip_data) = cache.get(&ip.text) {
        cached_ip_data.clone()
    } else {
        RipInfo::new(user_agent)?
            .fetch_api_data(&ip.text, &config)
            .await?
    };

    // print the data
    let json_to_print = serde_json::to_string(&ip_data)?;
    println!("{}", &json_to_print.to_colored_json_auto()?);

    // save the data on the cache file
    cache.insert(ip_data.ip.clone(), ip_data);
    update_cache(&cache)?;

    Ok(())
}
