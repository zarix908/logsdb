use skiplist::ordered_skiplist::OrderedSkipList;

use crate::log::Log;

pub trait Writer {
    fn write(self: &mut Self, memtable: OrderedSkipList<Log>) -> Result<(), String>;
}
