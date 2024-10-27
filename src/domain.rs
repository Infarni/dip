use std::net::IpAddr;

use regex::Regex;

use std::net::ToSocketAddrs;

use crate::constants;

use thiserror::Error;

pub struct Domain {
    pub value: String,
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid domain name: {0}")]
    InvalidDomain(String),

    #[error("Get ip by domain error: {0}")]
    GetIp(String),

    #[error("Ip not found")]
    IpNotFound,
}

pub type DomainResult<T> = Result<T, DomainError>;

impl Domain {
    pub fn new(value: &String) -> DomainResult<Self> {
        let object: Self = Self {
            value: value.clone(),
        };

        if object.is_valid() {
            Ok(object)
        } else {
            Err(DomainError::InvalidDomain(value.clone()))
        }
    }

    pub fn get_ip(&self) -> DomainResult<IpAddr> {
        match format!("{}:80", self.value).to_socket_addrs() {
            Ok(mut values) => match values.next() {
                Some(value) => Ok(value.ip()),
                None => Err(DomainError::IpNotFound),
            },
            Err(err) => Err(DomainError::GetIp(err.to_string())),
        }
    }

    pub fn is_valid(&self) -> bool {
        let pattern: Regex = Regex::new(constants::DOMAIN_PATTERN).unwrap();

        pattern.is_match(&self.value)
    }
}
