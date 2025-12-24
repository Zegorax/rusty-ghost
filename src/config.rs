use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub struct GhostConfig {
    /// Proxy listening port
    #[clap(short, long, default_value = "3773")]
    pub port: u16,

    /// Report download amount to always be zero
    #[clap(long)]
    pub zero_dl: bool,

    /// Adds the download amount to left
    #[clap(long)]
    pub intact_left: bool,

    /// Certificate authority private key path
    #[clap(long)]
    pub private_key: PathBuf,

    /// Certificate authority certificate path
    #[clap(long)]
    pub ca_cert: PathBuf,

    /// Do not check if the query path ends with /announce
    #[clap(long)]
    pub no_announce_path_check: bool,
}
