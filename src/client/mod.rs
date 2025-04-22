pub mod discovery;

pub fn send_discover() -> Result<(), Box<dyn std::error::Error>> {
    discovery::send_discover().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
} 