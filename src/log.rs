use std::cmp::{Ord, Ordering};

#[derive(PartialEq, Debug)]
pub struct Log {
    pub timestamp: u128,
    pub ip: [u8; 4],
    pub request: String,
}

impl Log {
    pub fn size(&self) -> u64 {
        20 + self.request.len() as u64
    }
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.timestamp.cmp(&other.timestamp);
        if ord != Ordering::Equal {
            return Some(ord);
        }

        Some(compare_ip(&self.ip, &other.ip))
    }
}

fn compare_ip(ip1: &[u8; 4], ip2: &[u8; 4]) -> Ordering {
    for i in 0..3 {
        let ord = ip1[i].cmp(&ip2[i]);
        if ord != Ordering::Equal {
            return ord;
        }
    }

    ip1[3].cmp(&ip2[3])
}
