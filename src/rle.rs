pub struct Encoder<V> {
    current: Option<V>,
    count: u64,
    output: Vec<u8>,
}

impl<V: PartialEq> Encoder<V> {
    pub fn new() -> Encoder<V> {
        Encoder {
            current: None,
            count: 0,
            output: Vec::new(),
        }
    }

    pub fn write(&mut self, item: V) {
        if let Some(current) = &self.current {
            if *current == item {
                self.count += 1;
                return;
            }
        }
    }
}
