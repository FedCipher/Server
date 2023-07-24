use std::sync::Mutex;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CommonState {
    /// The number of seconds that this instance has been online.
    pub uptime: Mutex<u64>
}
