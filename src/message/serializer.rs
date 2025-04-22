use super::DHCPMessage;

pub fn serialize(msg: &DHCPMessage) -> Result<String, serde_json::Error> {
    serde_json::to_string(msg)
}

pub fn deserialize(data: &str) -> Result<DHCPMessage, serde_json::Error> {
    serde_json::from_str(data)
}
