use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub domain: String,

    #[arg(short, long)]
    pub out: Option<PathBuf>,
}
