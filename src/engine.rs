use skiplist::ordered_skiplist::OrderedSkipList;
use std::mem;

use crate::fswriter::FsWriter;
use crate::log::Log;

pub struct Engine {
    size: u64,
    size_limit: u64,
    memtable: OrderedSkipList<Log>,
    writer: FsWriter,
}

impl Engine {
    pub fn new(size_limit: u64, writer: FsWriter) -> Engine {
        Engine {
            size: 0,
            size_limit,
            memtable: OrderedSkipList::new(),
            writer,
        }
    }

    pub fn insert(&mut self, log: Log) -> Result<(), String> {
        self.size += log.size();
        self.memtable.insert(log);

        if self.size > self.size_limit {
            let mut memtable = OrderedSkipList::new();

            mem::swap(&mut self.memtable, &mut memtable);
            self.writer
                .write(memtable)
                .map_err(|e| format!("write memtable to disk failed {}", e))?;
        }

        Ok(())
    }
}
