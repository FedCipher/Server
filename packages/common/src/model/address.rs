use serde::{Serialize, Deserialize, Deserializer, Serializer};
use regex::Regex;
use lazy_static::lazy_static;
use std::{fmt, error};
use serde::de;

use super::{Identifier, TypeConversionError};

/// A globally unique address.
#[derive(Debug, Clone)]
pub struct Address {
    /// The locally unique identifier for a piece of data on a host.
    pub id: Identifier,

    /// An IPv4 or IPv6 address or a domain name with at least one A or AAAA record pointing to a routable IP address.
    pub host: String
}

const HOST_PATTERN: &str = r"([a-zA-Z0-9_-]{32})@((?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z0-9][a-z0-9-]{0,61}[a-z0-9])";

#[derive(Debug)]
pub enum ParseError {
    Address(String),
    Identifier(TypeConversionError)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Address(value) => write!(f, "{} is not a valid address", value),
            ParseError::Identifier(error) => write!(f, "{}", error)
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::Address(_) => None,
            ParseError::Identifier(ref error) => Some(error)
        }
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(HOST_PATTERN).unwrap();
}

impl TryFrom<&str> for Address {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let captures = RE
            .captures(value)
            .ok_or(ParseError::Address(value.into()))?;
        let id = captures
            .get(1)
            .ok_or(ParseError::Address(value.into()))?
            .as_str();
        let host = captures
            .get(2)
            .ok_or(ParseError::Address(value.into()))?
            .as_str()
            .to_string();
        let address = Address {
            id: Identifier::try_from(id).map_err(ParseError::Identifier)?,
            host
        };

        Ok(address)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}@{}", self.id, self.host)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let address = self.to_string();

        String::serialize(&address, serializer)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let binding = String::deserialize(deserializer)?;
        let value = binding.as_str();
        let address = Address::try_from(value).map_err(de::Error::custom)?;

        Ok(address)
    }
}
