use actix_web::web::{Data, Json};
use actix_web::{get, Responder, Result};
use common::model::{Identifier, Address};

use crate::model::{SealedLetter, LetterAttachments, EmbeddedAttachment, RemoteAttachment};
use crate::state::MailState;

#[get("")]
pub async fn send_mail(state: Data<MailState>) -> Result<impl Responder> {
    let mut counter = state.total_sent_letters.lock().unwrap();

    *counter += 1;

    let sender = Address {
        id: Identifier::new(),
        host: String::from("example.com")
    };
    let recipients = vec![
        Address {
            id: Identifier::new(),
            host: String::from("example.com")
        }
    ];
    let embedded = vec![
        EmbeddedAttachment {
            id: Identifier::new(),
            size: 0,
            labels: Default::default(),
            data: vec![].into(),
            signature: Some(
                vec![].into()
            )
        }
    ];
    let remote = vec![
        RemoteAttachment {
            id: Identifier::new(),
            address: Address {
                id: Identifier::new(),
                host: String::from("example.com")
            },
            size: 0,
            labels: Default::default(),
            signature: Some(
                vec![].into()
            )
        }
    ];
    let attachments = LetterAttachments {
        embedded,
        remote
    };
    let letter = SealedLetter {
        id: Identifier::new(),
        sender: Some(sender),
        recipients,
        attachments: Some(attachments),
        labels: Default::default(),
        subject: Some(
            vec![].into()
        ),
        body: Some(
            vec![].into()
        ),
        signature: Some(
            vec![].into()
        )
    };

    let response = Json(letter);

    Ok(response)
}
