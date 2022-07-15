use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::vec::Vec;

use crate::log::Log;
use crate::memtable::Memtable;
use crate::store::Store;

pub struct FsStore {
    files: Vec<File>,
}

impl FsStore {
    pub fn new() -> Result<FsStore, String> {
        let mut files = Vec::new();

        for i in 0..4 {
            let filename = if i < 3 {
                format!("./data/column{}.cln", i)
            } else {
                String::from("./data/column2.blk")
            };

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename.clone())
                .map_err(|e| format!("open file {} failed: {}", filename, e))?;
            files.push(file);
        }

        Ok(FsStore { files })
    }
}

impl Store for FsStore {
    fn write(&mut self, memtable: Memtable<Log>) -> Result<(), String> {
        let mut offset = 0u64;

        for log in memtable {
            self.files[0]
                .write_all(&log.timestamp.to_le_bytes()[..])
                .map_err(|e| format!("write timestamp failed: {}", e))?;

            self.files[1]
                .write_all(&log.ip[..])
                .map_err(|e| format!("write ip failed: {}", e))?;

            self.files[2]
                .write_all(&log.request.as_bytes())
                .map_err(|e| format!("write request failed: {}", e))?;
            offset += log.request.len() as u64;
            self.files[3]
                .write_all(&offset.to_le_bytes())
                .map_err(|e| format!("write offset of request string failed: {}", e))?;
        }

        Ok(())
    }

    fn read(self: &Self) -> Result<Log, String> {
        Ok(Log {
            timestamp: 1,
            ip: [1, 1, 1, 1],
            request: String::from("asdkj"),
        })
    }
}
