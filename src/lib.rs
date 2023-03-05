pub mod ip_response;
pub mod parser;
mod ripinfo;

pub use ripinfo::*;

pub mod cache {
    use crate::ip_response::IpData;
    use std::collections::HashMap;
    pub type RipInfoCache = HashMap<String, IpData>;
    use anyhow::anyhow;
    use std::fs;
    extern crate directories;
    use directories::ProjectDirs;

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
}

pub fn print_usage() {
    println!(
        "{}",
        r#"USAGE: ripinfo [IP] [OPTIONS]...
OPTIONS                 DESCRIPTION
--help                  Prints this message
--firefox               Sets the user-agent to firefox
--edge                  Sets the user-agent to edge
"#
    )
}
