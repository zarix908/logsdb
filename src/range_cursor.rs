use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeCursor<T: Ord, P> {
    pub begin: T,
    pub end: T,
    pub pointer: RangeCursorPointer<P>,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RangeCursorPointer<T> {
    Mem(T),         // point to object in memtable
    Disk(Vec<u64>), // point to offset in cln or blk file
    Unset,          // not set yet
    Done,
}

impl<T: Ord + Clone, P> RangeCursor<T, P> {
    pub fn new(
        begin: T,
        end: T,
        page_size: u64,
        pointer: RangeCursorPointer<P>,
    ) -> RangeCursor<T, P> {
        RangeCursor {
            begin: begin.clone(),
            end,
            page_size,
            pointer: pointer,
        }
    }
}
