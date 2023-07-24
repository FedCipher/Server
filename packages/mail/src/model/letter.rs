use serde::{Serialize, Deserialize};
use common::model::{Identifier, Address, Labels, Blob};

use super::LetterAttachments;

/// A letter that has been partially encrypted by the client.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SealedLetter {
    /// The locally unique identifier of the letter.
    pub id: Identifier,

    /// The return address for the letter, such as in cases of undeliverable mail.
    pub sender: Option<Address>,

    /// The destination addresses for the letter.
    pub recipients: Vec<Address>,

    /// Any attachments to the letter.
    pub attachments: Option<LetterAttachments>,

    /// Any labels added to the letter.
    #[serde(default)]
    pub labels: Labels,

    /// The encrypted subject line of the letter.
    pub subject: Option<Blob>,

    /// The encrypted body of the letter.
    pub body: Option<Blob>,

    /// A digital signature for the letter.
    pub signature: Option<Blob>
}
