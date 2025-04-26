use std::net::Ipv4Addr;
use rand::Rng;

pub struct IpPool {
    start: Ipv4Addr,
    end: Ipv4Addr,
}

impl IpPool {
    pub fn new(start: &str, end: &str) -> Self {
        IpPool {
            start: start.parse().unwrap_or(Ipv4Addr::new(192, 168, 1, 100)),
            end: end.parse().unwrap_or(Ipv4Addr::new(192, 168, 1, 200)),
        }
    }

    pub fn get_random(&mut self) -> Option<Ipv4Addr> {
        let start = u32::from(self.start);
        let end = u32::from(self.end);
        
        if start >= end {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(start..=end);
        
        Some(Ipv4Addr::from(random))
    }
} 