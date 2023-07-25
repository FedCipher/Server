use serde::{Serialize, Deserialize};

use super::Identifier;

#[derive(Serialize, Deserialize, Debug)]
/// A report made concerning problematic data.
pub struct Report {
    /// The unique identifier for the problematic data.
    pub id: Identifier,

    /// The decryption key for the problematic data.
    pub key: String,

    /// An optional description of the problem.
    pub comment: Option<String>
}
