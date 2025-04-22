pub mod serializer;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DHCPMessage {
    pub msg_type: String,
    pub ip: String,
} 