use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};

use crate::size::Size;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Log {
    pub timestamp: u128,
    pub ip: [u8; 4],
    pub request: String,
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

impl Size for Log {
    fn size(&self) -> u64 {
        let request_and_ip_size = 20;
        request_and_ip_size + self.request.len() as u64
    }
}
