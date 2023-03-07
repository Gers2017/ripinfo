use crate::ip_response::IpData;
use anyhow::anyhow;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
extern crate directories;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "RipInfo";
const APPLICATION: &str = "ripinfo";
const CACHE_FILE: &str = "ripinfo.json";
const CONFIG_FILE: &str = "ripinfo_config.json";

pub type RipInfoCache = HashMap<String, IpData>;

fn get_config_dir() -> anyhow::Result<PathBuf> {
    let p = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or(anyhow!("error on project dirs"))?;
    Ok(p.config_dir().to_owned())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RipInfoConfig {
    pub use_token: bool,
    pub token: Option<String>,
}

impl Default for RipInfoConfig {
    fn default() -> Self {
        RipInfoConfig {
            use_token: false,
            token: None,
        }
    }
}

impl Display for RipInfoConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "use_token: {}\ntoken: {}",
            self.use_token,
            self.token.clone().unwrap_or_default()
        )
    }
}

impl RipInfoConfig {
    pub fn load_config() -> anyhow::Result<RipInfoConfig> {
        let config_dir = get_config_dir()?;

        if !&config_dir.exists() {
            fs::create_dir(&config_dir)?;
        }

        let mut config_file = config_dir;
        config_file.push(CONFIG_FILE);

        if !&config_file.exists() {
            RipInfoConfig::save_config(&RipInfoConfig::default())?;
        }

        let config = match fs::read_to_string(&config_file) {
            Ok(text) => {
                let config = serde_json::from_str::<RipInfoConfig>(&text).unwrap_or_default();
                Ok(config)
            }
            Err(e) => Err(anyhow!(e.to_string())),
        }?;

        Ok(config)
    }

    pub fn save_config(config: &RipInfoConfig) -> anyhow::Result<()> {
        let mut config_file = get_config_dir()?;
        config_file.push(CONFIG_FILE);

        let json_str = serde_json::to_string_pretty(config)?;
        fs::write(config_file, json_str)?;

        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use crate::config::RipInfoConfig;

    #[test]
    fn load_save_config_test() -> anyhow::Result<()> {
        // load initial config
        let x = RipInfoConfig::load_config()?;
        let mut config = x;
        config.use_token = true;

        // save config
        assert!(RipInfoConfig::save_config(&config).is_ok());

        // check saved changes
        let config = RipInfoConfig::load_config()?;
        assert_eq!(config.use_token, true);

        Ok(())
    }
}
