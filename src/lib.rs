pub mod config;
pub mod ip_response;
pub mod parser;
mod ripinfo;
pub use ripinfo::*;

pub fn print_usage() {
    println!(
        "{}",
        r#"USAGE: ripinfo [IP] [OPTIONS]...
OPTIONS                 DESCRIPTION
--help                  Prints this message
--firefox               Sets the user-agent to firefox
--edge                  Sets the user-agent to edge
--show-token            Prints the ripinfo config WITH THE API TOKEN and the current value of 'use-token'
--show-use-token        Prints the current value of 'use-token'
--toggle-token          Toggles the 'use-token' property inside ripinfo_config.json
"#
    )
}
