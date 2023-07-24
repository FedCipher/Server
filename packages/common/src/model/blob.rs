use serde::{Serialize, Deserialize, Deserializer, Serializer};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use serde::de::Error;

pub type OptionalBlob = Option<Blob>;

/// An arbitrary block of binary data.
#[derive(Debug, Clone)]
pub struct Blob(Vec<u8>);

impl From<Vec<u8>> for Blob {
    fn from(value: Vec<u8>) -> Self {
        Blob(value)
    }
}

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let bytes = &self.0;
        let base64 = URL_SAFE.encode(bytes);

        String::serialize(&base64, serializer)
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let base64 = String::deserialize(deserializer)?;

        let bytes = URL_SAFE.decode(base64).map_err(|error| {
            Error::custom(error)
        })?;

        Ok(
            Blob::from(bytes)
        )
    }
}
