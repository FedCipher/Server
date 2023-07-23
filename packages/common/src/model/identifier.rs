use std::fmt;
use std::error;
use serde::{Serialize, Deserialize, Deserializer, Serializer};
use base64::{Engine, DecodeError};
use base64::engine::general_purpose::URL_SAFE;
use serde::de;
use rand::{Rng, thread_rng};

#[derive(Debug)]
pub enum TypeConversionError {
    InvalidLength(usize),
    Decode(DecodeError)
}

impl fmt::Display for TypeConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeConversionError::InvalidLength(length) => write!(formatter, "expected 24 bytes but got {}", length),
            TypeConversionError::Decode(error) => write!(formatter, "{}", error)
        }
    }
}

impl error::Error for TypeConversionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TypeConversionError::InvalidLength(_) => None,
            TypeConversionError::Decode(ref error) => Some(error)
        }
    }
}

impl From<Vec<u8>> for TypeConversionError {
    fn from(value: Vec<u8>) -> Self {
        let length = value.len();

        TypeConversionError::InvalidLength(length)
    }
}

pub type UUID = [u8; 24];

/// A 24 byte (192 bit) unique identifier.
///
/// All identifiers must be locally unique.
#[derive(Debug)]
pub struct Identifier(UUID);

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uuid = self.0;
        let base64 = URL_SAFE.encode(uuid);

        write!(formatter, "{}", base64)
    }
}

impl From<UUID> for Identifier {
    fn from(value: UUID) -> Self {
        Identifier(value)
    }
}

impl Identifier {
    /// Generates a new random identifier.
    pub fn new() -> Self {
        let uuid = thread_rng().gen::<UUID>();

        Identifier::from(uuid)
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Identifier::new()
    }
}

impl TryFrom<Vec<u8>> for Identifier {
    type Error = TypeConversionError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let uuid: UUID = value.try_into().map_err(TypeConversionError::from)?;

        let identifier = Identifier(uuid);

        Ok(identifier)
    }
}

impl TryFrom<String> for Identifier {
    type Error = TypeConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes: Vec<u8> = URL_SAFE.decode(value).map_err(TypeConversionError::Decode)?;
        let identifier = Identifier::try_from(bytes)?;

        Ok(identifier)
    }
}

impl TryFrom<&str> for Identifier {
    type Error = TypeConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes: Vec<u8> = URL_SAFE.decode(value).map_err(TypeConversionError::Decode)?;
        let identifier = Identifier::try_from(bytes)?;

        Ok(identifier)
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let uuid = self.0;
        let base64 = URL_SAFE.encode(uuid);

        String::serialize(&base64, serializer)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let base64 = String::deserialize(deserializer)?;
        let identifier = Identifier::try_from(base64).map_err(de::Error::custom)?;

        Ok(identifier)
    }
}
