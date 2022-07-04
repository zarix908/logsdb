use skiplist::ordered_skiplist::OrderedSkipList;
use std::mem;

use crate::log::Log;

pub struct Engine {
    size: u64,
    size_limit: u64,
    memtable: OrderedSkipList<Log>,
}

impl Engine {
    pub fn new(size_limit: u64) -> Engine {
        Engine {
            size: 0,
            size_limit: size_limit,
            memtable: OrderedSkipList::new(),
        }
    }

    pub fn insert(&mut self, log: Log) {
        self.size += log.size();
        self.memtable.insert(log);

        if self.size > self.size_limit {
            let mut memtable = OrderedSkipList::new();

            mem::swap(&mut self.memtable, &mut memtable);
            for log in memtable {
                println!("{:?}", log);
            }
        }
    }
}
