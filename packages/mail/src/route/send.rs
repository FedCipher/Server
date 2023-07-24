use std::collections::HashSet;
use std::fmt;
use log::debug;
use actix_web::web::{Data, Json};
use actix_web::body::BoxBody;
use actix_web::{post, HttpResponse, Responder, Result, ResponseError};
use common::model::Labels;

use crate::model::{SealedLetter, LetterAttachments};
use crate::configuration::MailConfiguration;
use crate::state::MailState;

#[derive(Debug)]
pub enum ReceiveMailError {
    NoRecipients,
    AnonymousSender,
    Unsigned,
    UnsignedAttachments,
    NoSubject,
    NoBody,
    MissingLabels(HashSet<String>)
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
            }
        }
    }
}

impl ResponseError for ReceiveMailError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::Forbidden().body(self.to_string())
    }
}

/// Returns true if all required labels are present, otherwise returns false.
fn check_labels(required: &HashSet<String>, labels: &Labels) -> bool {
    required.iter().all(|value| labels.contains_key(value))
}

/// Returns true if all attachments are signed, or if there are no attachments, otherwise returns false.
fn check_attachments(attachments: LetterAttachments) -> bool {
    if attachments.embedded.is_empty() && attachments.remote.is_empty() {
        return true;
    }

    if attachments.embedded.iter().any(|value| value.signature.is_none()) {
        return false;
    }

    if attachments.remote.iter().any(|value| value.signature.is_none()) {
        return false;
    }

    true
}

fn verify_letter(letter: SealedLetter, configuration: Data<MailConfiguration>) -> Result<(), ReceiveMailError> {
    if letter.recipients.is_empty() {
        return Err(ReceiveMailError::NoRecipients);
    }

    if !configuration.accept.anomyous_sender && letter.sender.is_none() {
        return Err(ReceiveMailError::AnonymousSender);
    }

    if !configuration.accept.unsigned && letter.signature.is_none() {
        return Err(ReceiveMailError::Unsigned);
    }

    if !configuration.accept.unsigned_attachments && letter.attachments.is_some_and(check_attachments) {
        return Err(ReceiveMailError::UnsignedAttachments);
    }

    if configuration.require.subject && letter.subject.is_none() {
        return Err(ReceiveMailError::NoSubject);
    }

    if configuration.require.body && letter.body.is_none() {
        return Err(ReceiveMailError::NoBody);
    }

    let required_labels = &configuration.require.labels;

    if !required_labels.is_empty() && check_labels(required_labels, &letter.labels) {
        return Err(ReceiveMailError::MissingLabels(required_labels.clone()));
    }

    Ok(())
}

#[post("")]
pub async fn receive_mail(
    json: Json<SealedLetter>,
    configuration: Data<MailConfiguration>,
    state: Data<MailState>
) -> Result<impl Responder> {
    let letter = json.into_inner();

    verify_letter(letter.clone(), configuration)?;

    match letter.sender {
        Some(value) => debug!("Received a letter from {}", value),
        None => debug!("Received an anonymous letter")
    }

    let mut counter = state.total_received_letters.lock().unwrap();

    *counter += 1;

    Ok(HttpResponse::Ok())
}
