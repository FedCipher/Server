use log::info;
use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder, Result};
use common::model::{Identifier, Address, Labels};

use crate::model::{SealedLetter, LetterAttachments, EmbeddedAttachment, RemoteAttachment};

#[post("/api/v1/mail")]
pub async fn receive_mail(data: Json<SealedLetter>) -> impl Responder {
    let letter = data.into_inner();

    info!("{:?}", letter);

    HttpResponse::Ok()
}

#[get("/api/v1/mail")]
pub async fn send_mail() -> Result<impl Responder> {
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
            labels: None,
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
            labels: None,
            signature: Some(
                vec![].into()
            )
        }
    ];
    let attachments = LetterAttachments {
        embedded: Some(embedded),
        remote: Some(remote)
    };
    let letter = SealedLetter {
        id: Identifier::new(),
        sender: Some(sender),
        recipients,
        attachments: Some(attachments),
        labels: Some(
            Labels::from([
                (String::from(""), String::from(""))
            ])
        ),
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
