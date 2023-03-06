pub mod ip_response;
pub mod parser;
mod ripinfo;
pub mod config;
pub use ripinfo::*;

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
