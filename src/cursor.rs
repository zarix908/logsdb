use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Cursor {
    Mem(u128),      // point to timestamp in memtable
    Disk(Vec<u64>), // point to offset in cln or blk file
}
