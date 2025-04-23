use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DHCPMessage {
    pub msg_type: String,
    pub ip: String,
} 