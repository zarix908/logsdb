use crate::memtable::Memtable;

use crate::log::Log;

pub trait Store {
    fn write(self: &mut Self, memtable: Memtable<Log>) -> Result<(), String>;
    fn read(self: &Self) -> Result<Log, String>;
}
