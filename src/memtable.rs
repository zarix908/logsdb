use skiplist::ordered_skiplist::OrderedSkipList;

use crate::size::Size;

pub struct Memtable<T: PartialOrd + Size> {
    size: u64,
    memtable: OrderedSkipList<T>,
}

impl<T: PartialOrd + Size> Memtable<T> {
    pub fn new() -> Memtable<T> {
        Memtable {
            size: 0,
            memtable: OrderedSkipList::<T>::new(),
        }
    }

    pub fn insert(&mut self, item: T) {
        self.size += item.size();
        self.memtable.insert(item);
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl<T: PartialOrd + Size> IntoIterator for Memtable<T> {
    type Item = T;
    type IntoIter = skiplist::ordered_skiplist::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.memtable.into_iter()
    }
}
