use serde::{Serialize, Deserialize};

use super::Blob;

/// A signed binary ciphertext.
#[derive(Serialize, Deserialize, Debug)]
pub struct SignedStringCipher {
    /// Encrypted data.
    pub ciphertext: String,

    /// A digital signature.
    pub signature: String
}

// A binary ciphertext that might be signed.
#[derive(Serialize, Deserialize, Debug)]
pub struct SignableStringCipher {
    /// Encrypted data.
    pub ciphertext: String,

    /// An optional digital signature.
    pub signature: Option<String>
}

/// A signed binary ciphertext.
#[derive(Serialize, Deserialize, Debug)]
pub struct SignedBlobCipher {
    /// Encrypted data.
    pub ciphertext: Blob,

    /// A digital signature.
    pub signature: Blob
}

// A binary ciphertext that might be signed.
#[derive(Serialize, Deserialize, Debug)]
pub struct SignableBlobCipher {
    /// Encrypted data.
    pub ciphertext: Blob,

    /// An optional digital signature.
    pub signature: Option<Blob>
}
