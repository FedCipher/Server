use std::collections::HashSet;
use std::fmt;
use log::debug;
use actix_web::web::{block, Data, Json};
use actix_web::body::BoxBody;
use actix_web::{post, HttpResponse, Responder, Result, ResponseError};
use common::model::Labels;
use common::database::redis::{get_connection, R2D2Pool, RedisDatabaseError};
use redis::RedisError;
use redis::Commands;

use crate::model::{SealedLetter, LetterAttachments};
use crate::configuration::MailConfiguration;

#[derive(Debug)]
pub enum ReceiveMailError {
    NoRecipients,
    AnonymousSender,
    Unsigned,
    UnsignedAttachments,
    NoSubject,
    NoBody,
    MissingLabels(HashSet<String>),
    CreateRedisConnection(RedisDatabaseError),
    Increment(RedisError)
}

impl fmt::Display for ReceiveMailError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReceiveMailError::NoRecipients => {
                write!(formatter, "At least one recipient must be provided")
            },
            ReceiveMailError::AnonymousSender => {
                write!(formatter, "Anonymous senders are forbidden")
            },
            ReceiveMailError::Unsigned => {
                write!(formatter, "Unsigned letters are forbidden")
            }
            ReceiveMailError::UnsignedAttachments => {
                write!(formatter, "Unsigned attachments are forbidden")
            },
            ReceiveMailError::NoSubject => {
                write!(formatter, "A letter subject is required")
            },
            ReceiveMailError::NoBody => {
                write!(formatter, "A letter body is required")
            },
            ReceiveMailError::MissingLabels(value) => {
                write!(formatter, "The following labels are required: {:#?}", value)
            },
            ReceiveMailError::CreateRedisConnection(error) => {
                write!(formatter, "{}", error)
            },
            ReceiveMailError::Increment(error) => {
                write!(formatter, "{}", error)
            }
        }
    }
}

impl ResponseError for ReceiveMailError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match &self {
            ReceiveMailError::CreateRedisConnection(_) | ReceiveMailError::Increment(_) => {
                HttpResponse::InternalServerError().body(self.to_string())
            },
            _ => {
                HttpResponse::Forbidden().body(self.to_string())
            }
        }
    }
}

/// The result of label validation.
#[derive(PartialEq)]
enum LabelsValidationResult {
    None,
    Invalid(HashSet<String>),
    Valid
}

/// Returns true if all required labels are present, otherwise returns false.
fn validate_labels(required_labels: &HashSet<String>, labels: Labels) -> LabelsValidationResult {
    if required_labels.is_empty() && labels.is_empty() {
        return LabelsValidationResult::None;
    }

    if required_labels.iter().all(|value| labels.contains_key(value)) {
        return LabelsValidationResult::Valid;
    }

    let keys = labels.keys().cloned().collect();
    let complement = required_labels - &keys;

    LabelsValidationResult::Invalid(complement)
}

/// The result of attachment validation.
#[derive(PartialEq)]
enum AttachmentValidationResult {
    None,
    Invalid,
    Valid
}

fn validate_attachments(attachments: Option<LetterAttachments>) -> AttachmentValidationResult {
    if attachments.is_none() {
        return AttachmentValidationResult::None;
    }

    let unwrapped = attachments.unwrap();

    if unwrapped.embedded.is_empty() && unwrapped.remote.is_empty() {
        return AttachmentValidationResult::None;
    }

    if unwrapped.embedded.iter().any(|value| value.signature.is_none()) {
        return AttachmentValidationResult::Invalid;
    }

    if unwrapped.remote.iter().any(|value| value.signature.is_none()) {
        return AttachmentValidationResult::Invalid;
    }

    AttachmentValidationResult::Valid
}

fn validate_letter(letter: SealedLetter, configuration: Data<MailConfiguration>) -> Result<(), ReceiveMailError> {
    if letter.recipients.is_empty() {
        return Err(ReceiveMailError::NoRecipients);
    }

    if !configuration.accept.anomyous_sender && letter.sender.is_none() {
        return Err(ReceiveMailError::AnonymousSender);
    }

    if !configuration.accept.unsigned && letter.signature.is_none() {
        return Err(ReceiveMailError::Unsigned);
    }

    let attachments = letter.attachments.clone();

    if !configuration.accept.unsigned_attachments {
        let result = validate_attachments(attachments);

        if result == AttachmentValidationResult::Invalid {
            return Err(ReceiveMailError::UnsignedAttachments);
        }
    }

    if configuration.require.subject && letter.subject.is_none() {
        return Err(ReceiveMailError::NoSubject);
    }

    if configuration.require.body && letter.body.is_none() {
        return Err(ReceiveMailError::NoBody);
    }

    let required_labels = configuration.require.labels.clone();

    if !required_labels.is_empty() {
        let labels = letter.labels;

        return match validate_labels(&required_labels, labels) {
            LabelsValidationResult::None => {
                Err(ReceiveMailError::MissingLabels(required_labels))
            },
            LabelsValidationResult::Invalid(complement) => {
                Err(ReceiveMailError::MissingLabels(complement))
            },
            LabelsValidationResult::Valid => {
                Ok(())
            }
        };
    }

    Ok(())
}

const TOTAL_RECEIVED_LETTERS: &str = "TOTAL_RECEIVED_LETTERS";

fn increment(pool: Data<R2D2Pool>) -> Result<(), ReceiveMailError> {
    let mut connection = get_connection(&pool).map_err(ReceiveMailError::CreateRedisConnection)?;

    connection.incr::<&str, u64, ()>(TOTAL_RECEIVED_LETTERS, 1).map_err(ReceiveMailError::Increment)?;

    Ok(())
}

#[post("")]
pub async fn receive_mail(
    json: Json<SealedLetter>,
    configuration: Data<MailConfiguration>,
    pool: Data<R2D2Pool>
) -> Result<impl Responder> {
    let letter = json.into_inner();

    validate_letter(letter.clone(), configuration)?;

    match letter.sender {
        Some(value) => debug!("Received a letter from {}", value),
        None => debug!("Received an anonymous letter")
    }

    block(|| increment(pool)).await??;

    Ok(HttpResponse::Ok())
}
