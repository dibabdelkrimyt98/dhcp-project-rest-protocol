pub mod discovery;
use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    discovery::run(config).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
} 