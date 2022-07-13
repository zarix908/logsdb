use skiplist::ordered_skiplist::OrderedSkipList;
use std::mem;

use crate::log::Log;
use crate::store::Store;

pub struct Engine<T: Store> {
    size: u64,
    size_limit: u64,
    memtable: OrderedSkipList<Log>,
    store: T,
}

impl<T: Store> Engine<T> {
    pub fn new(size_limit: u64, store: T) -> Engine<T> {
        Engine {
            size: 0,
            size_limit,
            memtable: OrderedSkipList::new(),
            store,
        }
    }

    pub fn insert(&mut self, log: Log) -> Result<(), String> {
        self.size += log.size();
        self.memtable.insert(log);

        if self.size > self.size_limit {
            let mut memtable = OrderedSkipList::new();

            mem::swap(&mut self.memtable, &mut memtable);
            self.store
                .write(memtable)
                .map_err(|e| format!("write memtable to disk failed {}", e))?;
        }

        Ok(())
    }
}
