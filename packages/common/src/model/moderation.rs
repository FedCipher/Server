use super::Identifier;

/// A report made by a user concerning a remote resource.
pub struct Report {
    /// The unique identifier for the problematic parcel.
    pub id: Identifier,
    pub key: String,
    pub comment: Option<String>
}
