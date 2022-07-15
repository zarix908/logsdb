use skiplist::ordered_skiplist::OrderedSkipList;
use std::mem;
use std::sync::{Condvar, Mutex};

use crate::log::Log;
use crate::store::Store;

pub struct Engine {
    memtable_is_full: Condvar,
    engine: Mutex<EngineInternal>,
    size_limit: u64,
}

struct EngineInternal {
    size: u64,
    memtable: OrderedSkipList<Log>,
}

impl Engine {
    pub fn new(size_limit: u64) -> Engine {
        Engine {
            engine: Mutex::new(EngineInternal {
                size: 0,
                memtable: OrderedSkipList::new(),
            }),
            size_limit: size_limit,
            memtable_is_full: Condvar::new(),
        }
    }

    pub fn run<S: Store>(&self, mut store: S) {
        loop {
            let memtable = self.swap_memtable();

            let result = store.write(memtable);
            if let Err(err) = result {
                log::error!("dump memtable failed: {}", err);
            }
        }
    }

    pub fn insert(&self, log: Log) {
        let memtable_size: u64;

        let mut engine = self.engine.lock().unwrap();
        engine.size += log.size();
        memtable_size = engine.size;
        engine.memtable.insert(log);
        drop(engine);

        if memtable_size > self.size_limit {
            self.memtable_is_full.notify_one();
        }
    }

    fn swap_memtable(&self) -> OrderedSkipList<Log> {
        let mut memtable: OrderedSkipList<Log>;

        let mut engine = self.engine.lock().unwrap();
        loop {
            if engine.size > self.size_limit {
                memtable = OrderedSkipList::new();
                mem::swap(&mut engine.memtable, &mut memtable);
                break;
            } else {
                engine = self.memtable_is_full.wait(engine).unwrap();
            }
        }

        return memtable;
    }
}
