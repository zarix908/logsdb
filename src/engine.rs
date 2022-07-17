use std::mem;
use std::ops::{Bound, DerefMut};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Condvar, Mutex,
};

use crate::log::Log;
use crate::memtable::Memtable;
use crate::range_cursor::{RangeCursor, RangeCursorPointer};
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

    pub fn read(&self, cursor: &mut RangeCursor<u128, Log>) -> Vec<Log> {
        let mut result = Vec::new();

        loop {
            let (continue_read, new_pointer) = match &cursor.pointer {
                RangeCursorPointer::Disk(_) => (false, RangeCursorPointer::Done),
                RangeCursorPointer::Mem(begin) => {
                    self.read_from_mem(Bound::Included(begin), &mut result, cursor)
                }
                RangeCursorPointer::Unset => {
                    self.read_from_mem(Bound::Unbounded, &mut result, cursor)
                }
                RangeCursorPointer::Done => (false, RangeCursorPointer::Done),
            };

            cursor.pointer = new_pointer;
            if !continue_read {
                break;
            }
        }

        result
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

    fn read_from_mem(
        &self,
        min: Bound<&Log>,
        result: &mut Vec<Log>,
        cursor: &RangeCursor<u128, Log>,
    ) -> (bool, RangeCursorPointer<Log>) {
        let memtable = self.memtable.lock().unwrap();

        for log in memtable.range(min, Bound::Unbounded) {
            if result.len() as u64 == cursor.page_size {
                return (false, RangeCursorPointer::Mem(log.clone()));
            }
            if log.timestamp > cursor.end {
                return (false, RangeCursorPointer::Done);
            }

            result.push(log.clone());
        }

        (true, RangeCursorPointer::Disk(vec![0, 0, 0]))
    }
}
