use skiplist::ordered_skiplist::OrderedSkipList;

use crate::log::Log;

pub trait Store {
    fn write(self: &mut Self, memtable: OrderedSkipList<Log>) -> Result<(), String>;
    fn read(self: &Self) -> Result<Log, String>;
}
