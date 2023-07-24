use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MailAccept {
    /// Whether to accept letters without a return address.
    pub anomyous_sender: bool,

    /// Whether to accept unsigned letters.
    pub unsigned: bool,

    /// Whether to accept unsigned letter attachments.
    pub unsigned_attachments: bool
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MailRequire {
    /// Require letters to possess a subject line.
    pub subject: bool,

    /// Require letters to possess a body.
    pub body: bool,

    /// Any letter labels that must be present.
    pub labels: HashSet<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailLimit {
    /// The maximum number of letter recipients.
    pub recipients: u64,

    /// The maximum subject line size in bytes.
    pub subject_size: u64,

    /// The maximum body size in bytes.
    pub body_size: u64,

    /// The maximum number of embedded attachments.
    pub embedded_attachments: u64,

    /// The maximum size in bytes of an embedded attachment.
    pub embedded_attachment_size: u64,

    /// The maximum number of remote attachments.
    pub remote_attachments: u64,

    /// The maximum size in bytes of a remote attachment.
    pub remote_attachment_size: u64,

    /// The maximum number of labels a letter can have.
    pub labels: u64
}

/// Attempt to set reasonable default limits.
impl Default for MailLimit {
    fn default() -> Self {
        Self {
            recipients: 100,
            subject_size: 1024,
            body_size: 262144,
            embedded_attachments: 100,
            embedded_attachment_size: 262144,
            remote_attachments: 100,
            remote_attachment_size: 536870912,
            labels: 1000
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MailConfiguration {
    /// Accepted mail data.
    pub accept: MailAccept,

    /// Required mail data.
    pub require: MailRequire,

    /// Limitations for letters.
    pub limit: MailLimit
}
