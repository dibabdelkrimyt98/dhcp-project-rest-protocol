use std::collections::VecDeque;
use std::net::Ipv4Addr;
use rand::seq::SliceRandom;

pub struct IpPool {
    pub available: VecDeque<Ipv4Addr>,
}

impl IpPool {
    pub fn new(start: &str, end: &str) -> Self {
        let start_ip: Ipv4Addr = start.parse().unwrap();
        let end_ip: Ipv4Addr = end.parse().unwrap();

        let start_octets = start_ip.octets();
        let end_octets = end_ip.octets();

        let mut ips = VecDeque::new();
        for i in start_octets[3]..=end_octets[3] {
            ips.push_back(Ipv4Addr::new(start_octets[0], start_octets[1], start_octets[2], i));
        }

        Self { available: ips }
    }

    pub fn get_random(&mut self) -> Option<Ipv4Addr> {
        let mut vec: Vec<_> = self.available.drain(..).collect();
        vec.shuffle(&mut rand::thread_rng());
        let ip = vec.pop();
        for ip in vec {
            self.available.push_back(ip);
        }
        ip
    }
}
