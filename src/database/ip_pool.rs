use std::net::Ipv4Addr;
use std::collections::VecDeque;

pub struct IpPool {
    available: VecDeque<Ipv4Addr>,
}

impl IpPool {
    pub fn new(start: Ipv4Addr, end: Ipv4Addr) -> Self {
        let mut available = VecDeque::new();
        let mut current = u32::from(start);
        let end = u32::from(end);

        while current <= end {
            available.push_back(Ipv4Addr::from(current));
            current += 1;
        }

        Self { available }
    }

    pub fn get_next(&mut self) -> Option<Ipv4Addr> {
        self.available.pop_front()
    }

    pub fn release(&mut self, ip: Ipv4Addr) {
        if !self.available.contains(&ip) {
            self.available.push_back(ip);
        }
    }
}
