use serde::{Serialize, Deserialize};

use common::model::{Identifier, Address, Labels, Blob};

/// A sealed attachment that was provided as part of a letter.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddedAttachment {
    /// A locally unique identifier for this attachment.
    pub id: Identifier,

    /// The declared size of the attachment.
    pub size: u64,

    /// Any attachment labels.
    #[serde(default)]
    pub labels: Labels,

    /// The encrypted attachment data.
    pub data: Blob,

    /// A digital signature for the attachment.
    pub signature: Option<Blob>
}

/// A sealed attachment that is known to exist only on a remote host.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteAttachment {
    /// A locally unique identifier for this attachment.
    pub id: Identifier,

    /// The globally unique address of the attachment.
    pub address: Address,

    /// The declared size of the attachment.
    pub size: u64,

    /// Any attachment labels.
    #[serde(default)]
    pub labels: Labels,

    /// A digital signature for the attachment.
    pub signature: Option<Blob>
}

/// A sealed attachment that exists on the local host.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalAttachment {
    /// A locally unique identifier for this attachment.
    pub id: Identifier,

    /// The globally unique address where the attachment came from.
    pub address: Address,

    /// The declared size of the attachment.
    pub size: u64,

    /// Any attachment labels.
    #[serde(default)]
    pub labels: Labels,

    /// The encrypted attachment data.
    pub data: Blob,

    /// A digital signature for the attachment.
    pub signature: Option<Blob>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LetterAttachments {
    /// All embedded attachments for a letter.
    #[serde(default)]
    pub embedded: Vec<EmbeddedAttachment>,

    /// All remote attachments for a letter.
    #[serde(default)]
    pub remote: Vec<RemoteAttachment>
}
