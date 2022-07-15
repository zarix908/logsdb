use std::mem;
use std::ops::DerefMut;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Condvar, Mutex,
};

use crate::log::Log;
use crate::memtable::Memtable;
use crate::store::Store;

pub struct Engine {
    flush_memtable: Condvar,
    memtable: Mutex<Memtable<Log>>,
    memtable_size_limit: u64,
    stopped: AtomicBool,
}

impl Engine {
    pub fn new(memtable_size_limit: u64) -> Engine {
        Engine {
            memtable: Mutex::new(Memtable::new()),
            memtable_size_limit: memtable_size_limit,
            flush_memtable: Condvar::new(),
            stopped: AtomicBool::new(false),
        }
    }

    pub fn run<S: Store>(&self, mut store: S) {
        loop {
            let memtable = self.swap_memtable();

            let result = store.write(memtable);
            if let Err(err) = result {
                log::error!("flush memtable failed: {}", err);
            }

            if self.stopped.load(Ordering::SeqCst) {
                break;
            }
        }
    }

    pub fn insert(&self, log: Log) {
        let memtable_size: u64;

        let mut memtable = self.memtable.lock().unwrap();
        memtable_size = memtable.size();
        memtable.insert(log);
        drop(memtable);

        if memtable_size > self.memtable_size_limit {
            self.flush_memtable.notify_one();
        }
    }

    pub fn stop(&self) {
        self.stopped.store(true, Ordering::SeqCst);
        self.flush_memtable.notify_one();
    }

    fn swap_memtable(&self) -> Memtable<Log> {
        let mut old_memtable: Memtable<Log>;

        let mut memtable = self.memtable.lock().unwrap();
        loop {
            if memtable.size() > self.memtable_size_limit || self.stopped.load(Ordering::SeqCst) {
                old_memtable = Memtable::new();
                mem::swap(memtable.deref_mut(), &mut old_memtable);
                break;
            }

            memtable = self.flush_memtable.wait(memtable).unwrap();
        }

        return old_memtable;
    }
}
