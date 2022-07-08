use skiplist::ordered_skiplist::OrderedSkipList;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::vec::Vec;

use crate::log::Log;
use crate::writer::Writer;

pub struct FsWriter {
    files: Vec<File>,
}

impl FsWriter {
    pub fn new() -> Result<FsWriter, String> {
        let mut files = Vec::new();

        for i in 0..3 {
            let filename = format!("./data/column{}.cln", i);
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename.clone())
                .map_err(|e| format!("open file {} failed: {}", filename, e))?;
            files.push(file);
        }

        Ok(FsWriter { files })
    }
}

impl Writer for FsWriter {
    fn write(&mut self, memtable: OrderedSkipList<Log>) -> Result<(), String> {
        for log in memtable {
            self.files[0]
                .write_all(&log.timestamp.to_le_bytes()[..])
                .map_err(|e| format!("write timestamp failed: {}", e))?;

            self.files[1]
                .write_all(&log.ip[..])
                .map_err(|e| format!("write ip failed: {}", e))?;

            let mut bytes = log.request.as_bytes().to_vec();
            bytes.push(0);
            self.files[2]
                .write_all(&bytes.as_slice())
                .map_err(|e| format!("write request failed: {}", e))?;
        }

        Ok(())
    }
}
