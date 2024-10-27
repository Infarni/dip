use std::{net::IpAddr, process};

use clap::Parser;
use dip::{config::Config, domain::Domain, ip::Ip};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config: Config = Config::parse();

    let domain: Domain = Domain::new(&config.domain).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let ip_addr: IpAddr = domain.get_ip().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let ip: Ip = Ip::new(ip_addr).await.unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    if let Some(value) = config.out {
        ip.save(&value)?;
    } else {
        println!("{:#?}", ip);
    }

    Ok(())
}
