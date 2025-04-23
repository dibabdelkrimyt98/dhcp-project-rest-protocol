use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{SystemTime, Duration};

pub struct RateLimiter {
    requests: HashMap<SocketAddr, SystemTime>,
    interval: Duration,
}

impl RateLimiter {
    pub fn new(interval_secs: u64) -> Self {
        Self {
            requests: HashMap::new(),
            interval: Duration::new(interval_secs, 0),
        }
    }

    pub fn allow(&mut self, addr: SocketAddr) -> bool {
        let now = SystemTime::now();
        match self.requests.get(&addr) {
            Some(last_time) => {
                if now.duration_since(*last_time).unwrap_or_default() >= self.interval {
                    self.requests.insert(addr, now);
                    true
                } else {
                    false
                }
            }
            None => {
                self.requests.insert(addr, now);
                true
            }
        }
    }
}
