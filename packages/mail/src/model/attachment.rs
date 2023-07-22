use serde::{Serialize, Deserialize};

use common::model::{Identifier, Address, Labels, Blob};

/// A sealed attachment that was provided as part of a letter.
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddedAttachment {
    /// A locally unique identifier for this attachment.
    pub id: Identifier,

    /// The declared size of the attachment.
    pub size: u64,

    /// Any attachment labels.
    pub labels: Option<Labels>,

    /// The encrypted attachment data.
    pub data: Blob,

    /// A digital signature for the attachment.
    pub signature: Option<Blob>
}

/// A sealed attachment that is known to exist only on a remote host.
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteAttachment {
    /// A locally unique identifier for this attachment.
    pub id: Identifier,

    /// The globally unique address of the attachment.
    pub address: Address,

    /// The declared size of the attachment.
    pub size: u64,

    /// Any attachment labels.
    pub labels: Option<Labels>,

    /// A digital signature for the attachment.
    pub signature: Option<Blob>
}

/// A sealed attachment that exists on the local host.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalAttachment {
    /// A locally unique identifier for this attachment.
    pub id: Identifier,

    /// The globally unique address where the attachment came from.
    pub address: Address,

    /// The declared size of the attachment.
    pub size: u64,

    /// Any attachment labels.
    pub labels: Option<Labels>,

    /// The encrypted attachment data.
    pub data: Blob,

    /// A digital signature for the attachment.
    pub signature: Option<Blob>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LetterAttachments {
    /// All embedded attachments for a letter.
    pub embedded: Option<Vec<EmbeddedAttachment>>,

    /// All remote attachments for a letter.
    pub remote: Option<Vec<RemoteAttachment>>,
}
