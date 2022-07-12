use std::mem;

pub struct RleEncoded<V> {
    item: V,
    count: u16,
}

pub struct RleEncoder<V> {
    current: Option<V>,
    count: u16,
    output: Vec<RleEncoded<V>>,
}

impl<V: PartialEq> RleEncoder<V> {
    pub fn new() -> RleEncoder<V> {
        RleEncoder {
            current: None,
            count: 0,
            output: Vec::new(),
        }
    }

    pub fn write(&mut self, mut item: V) {
        if let Some(current) = &mut self.current {
            if *current == item && self.count < u16::MAX {
                self.count += 1;
                return;
            }

            mem::swap(current, &mut item);
            self.output.push(RleEncoded {
                item,
                count: self.count,
            });
            self.count = 1;

            return;
        }

        self.count = 1;
        self.current = Some(item);
    }

    pub fn flush(mut self) -> Vec<RleEncoded<V>> {
        if let Some(current) = self.current {
            self.output.push(RleEncoded {
                item: current,
                count: self.count,
            });
        }

        self.output
    }
}

#[cfg(test)]
mod tests {
    use super::RleEncoder;
    use std::collections::HashMap;

    #[test]
    fn run_rle() {
        let mut encoder = RleEncoder::new();

        for b in String::from("aaa78bbcdddd").bytes() {
            encoder.write(b);
        }
        let output = encoder.flush();

        let counts = HashMap::from([(97, 3), (55, 1), (56, 1), (98, 2), (99, 1), (100, 4)]);
        for encoded in output {
            assert_eq!(counts.get(&encoded.item).unwrap(), &encoded.count)
        }
    }
}
