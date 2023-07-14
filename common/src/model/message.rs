use std::collections::HashMap;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Letter {
    /// The 12-byte unique identifier of the letter.
    pub id: String,

    /// The return address for the letter.
    pub sender: Option<String>,

    /// The destination addresses for the letter.
    pub recipients: Vec<String>,

    /// The subject line of the letter.
    pub subject: Option<String>,

    /// The content of the letter.
    pub body: Option<String>,

    /// All attachments to the letter.
    ///
    /// A key is a content identifier,
    /// and a value is a link to a remote resource that may be fetched.
    pub attachments: Option<HashMap<String, String>>,

    /// Any labels added to the letter.
    pub labels: Option<HashMap<String, String>>
}
