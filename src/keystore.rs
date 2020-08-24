use crate::key;

use rmps::decode;
use sodiumoxide::crypto::secretbox;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct KeyStore {
    keys: HashMap<String, key::RawKey>,
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
    Format(String),
}

impl KeyStore {
    pub fn new<T: AsRef<Path>>(store_file: T) -> Result<KeyStore, KSLoadFail> {
        return Ok(decode::from_read(BufReader::new(File::open(store_file)?))?);
    }
}
