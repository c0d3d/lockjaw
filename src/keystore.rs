use crate::key;

use rmps::{decode, encode};
use sodiumoxide::crypto::secretbox;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Seek, SeekFrom, Write};
use std::ops::Drop;
use std::path::Path;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
struct KeyStoreData(HashMap<String, key::RawKey>);

pub struct KeyStore {
    file: File,
    keys: KeyStoreData,
}

impl From<std::io::Error> for KSLoadFail {
    fn from(e: std::io::Error) -> KSLoadFail {
        return KSLoadFail::IO(e.kind());
    }
}

impl From<decode::Error> for KSLoadFail {
    fn from(e: decode::Error) -> KSLoadFail {
        return KSLoadFail::Format(format!("{}", e));
    }
}

#[derive(Debug)]
pub enum KSLoadFail {
    IO(std::io::ErrorKind),
    BadPath,
    Format(String),
}

impl KeyStore {
    pub fn new<T: AsRef<Path>>(store_file: T) -> Result<KeyStore, KSLoadFail> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(store_file)?;

        return Ok(KeyStore {
            keys: decode::from_read(BufReader::new(file.try_clone()?))?,
            file,
        });
    }

    pub fn save_keystore(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(
            &encode::to_vec(&self.keys)
                .or(Err(io::Error::new(io::ErrorKind::Other, "Encode failed!")))?,
        )
    }
}

impl Drop for KeyStore {
    fn drop(&mut self) {
        self.save_keystore().expect("Drop save failed!");
    }
}
