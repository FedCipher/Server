use std::fmt;
use actix_web::body::BoxBody;
use actix_web::web::{block, Data, Json};
use actix_web::{get, Responder, Result, ResponseError, HttpResponse};
use common::model::{Identifier, Address};
use common::database::redis::{get_connection, R2D2Pool, RedisDatabaseError};
use redis::RedisError;
use redis::Commands;

use crate::model::{SealedLetter, LetterAttachments, EmbeddedAttachment, RemoteAttachment};

const TOTAL_SENT_LETTERS: &str = "TOTAL_SENT_LETTERS";

#[derive(Debug)]
pub enum SendMailError {
    CreateRedisConnection(RedisDatabaseError),
    Increment(RedisError)
}

impl fmt::Display for SendMailError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SendMailError::CreateRedisConnection(error) => {
                write!(formatter, "{}", error)
            },
            SendMailError::Increment(error) => {
                write!(formatter, "{}", error)
            }
        }
    }
}

impl ResponseError for SendMailError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::InternalServerError().body(self.to_string())
    }
}

fn increment(pool: Data<R2D2Pool>) -> Result<(), SendMailError> {
    let mut connection = get_connection(&pool).map_err(SendMailError::CreateRedisConnection)?;

    connection.incr::<&str, u64, ()>(TOTAL_SENT_LETTERS, 1).map_err(SendMailError::Increment)?;

    Ok(())
}

#[get("")]
pub async fn send_mail(pool: Data<R2D2Pool>) -> Result<impl Responder> {
    block(|| increment(pool)).await??;

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
