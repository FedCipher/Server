use std::sync::Mutex;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MailState {
    /// How many letters this address has received.
    pub total_received_letters: Mutex<u64>,

    /// How many letters this address has sent.
    pub total_sent_letters: Mutex<u64>
}
