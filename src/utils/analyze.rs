use std::process::Command;
use serde_json::Value;

pub fn parse_capture_to_json(filename: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("tshark")
        .args(["-r", filename, "-T", "json"])
        .output()?;

    let json = String::from_utf8(output.stdout)?;
    let parsed: Value = serde_json::from_str(&json)?;
    Ok(parsed)
}
