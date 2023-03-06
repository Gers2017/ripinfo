use crate::ip_response::IpData;
use anyhow::anyhow;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
extern crate directories;
use directories::ProjectDirs;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "RipInfo";
const APPLICATION: &str = "ripinfo";
const CACHE_FILE: &str = "ripinfo.json";

pub type RipInfoCache = HashMap<String, IpData>;

fn get_config_dir() -> anyhow::Result<PathBuf> {
    let p = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or(anyhow!("error on project dirs"))?;
    Ok(p.config_dir().to_owned())
}

pub fn load_cache() -> anyhow::Result<RipInfoCache> {
    let config_dir = get_config_dir()?;

    if !&config_dir.exists() {
        fs::create_dir(&config_dir)?;
    }

    let mut cache_file = config_dir;
    cache_file.push(CACHE_FILE);

    match fs::read_to_string(&cache_file) {
        Ok(text) => Ok(serde_json::from_str::<RipInfoCache>(&text)?),
        Err(_e) => Ok(HashMap::new()),
    }
}

pub fn update_cache(cache: &RipInfoCache) -> anyhow::Result<()> {
    let mut cache_file = get_config_dir()?;
    cache_file.push(CACHE_FILE);

    let json_str = serde_json::to_string(cache)?;
    fs::write(&cache_file, json_str)?;

    Ok(())
}
