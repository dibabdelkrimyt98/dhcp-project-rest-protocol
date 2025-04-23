// src/config.rs
use std::env;

#[derive(Debug)]
pub enum NetworkMode {
    TestLocalhost,
    RealNetwork,
    VirtualMachine,
    SimulatedTraffic,
}

#[derive(Debug)]
pub struct Config {
    pub interface: String,
    pub port: u16,
    pub mode: String,
}

impl Config {
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        
        // Default values
        let mut interface = "0.0.0.0".to_string();
        let mut port = 6767;
        let mut mode = "server".to_string();
        
        // Parse command line arguments
        for i in 1..args.len() {
            if args[i] == "--interface" && i + 1 < args.len() {
                interface = args[i + 1].clone();
            } else if args[i] == "--port" && i + 1 < args.len() {
                port = args[i + 1].parse().unwrap_or(6767);
            } else if args[i] == "--mode" && i + 1 < args.len() {
                mode = args[i + 1].clone();
            }
        }
        
        Config {
            interface,
            port,
            mode,
        }
    }
}
