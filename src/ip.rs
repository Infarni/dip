use std::{fs::File, io::Write, net::IpAddr, path::PathBuf};

use reqwest::Response;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ip {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    org: Option<String>,
}

#[derive(Debug, Error)]
pub enum IpError {
    #[error("Internet connection error")]
    Request(#[from] reqwest::Error),
}

pub type IpResult<T> = Result<T, IpError>;

impl Ip {
    pub async fn new(value: IpAddr) -> IpResult<Self> {
        let url = format!("https://ipinfo.io/{}/json", value.to_string());

        let response: Response = reqwest::get(&url).await?;

        let ip: Self = response.json().await?;

        Ok(ip)
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let content: String = serde_json::to_string_pretty(&self).unwrap();

        let mut file: File = File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
