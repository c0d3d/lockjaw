use crate::databuff::DataBuff;
use crate::databuff::MAX_PAYLOAD_SIZE;
use crate::memlock::{self, MLock};
use rmps::{decode, encode};
use serde;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::hash::sha256;
use sodiumoxide::crypto::secretbox;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Seek, SeekFrom, Write};
use std::ops::DerefMut;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct KeyStoreData(HashMap<String, RawKey>);

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

#[derive(Debug)]
pub enum KSAddFail {
    DataTooLarge(usize),
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

    pub fn add_key_interactive<Name>(
        &mut self,
        ktype: KeyType,
        name: &Name,
        value: &[u8],
    ) -> Option<KSAddFail>
    where
        Name: AsRef<str>,
    {
        if value.len() > MAX_PAYLOAD_SIZE {
            return Some(KSAddFail::DataTooLarge(value.len()));
        }

        unimplemented!();
    }
}

type StrErr<T> = Result<T, &'static str>;
type LockedBuff<T> = MLock<T>;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum KeyType {
    OTC,
    PASSWORD,
}

#[derive(Serialize, Deserialize)]
pub struct RawKey {
    ktype: KeyType,
    tag: secretbox::Tag,
    nonce: secretbox::Nonce,
    enc_payload: memlock::MLock<DataBuff>,
}

impl RawKey {
    fn create_interactive(ktype: KeyType, value: &[u8]) -> Result<RawKey, String> {
        let mut locked_buff = memlock::MLock::new(
            DataBuff::from_slice(value).ok_or("Couldnt consume data".to_string())?,
        )?;

        let nonce = secretbox::gen_nonce();
        let key = secretbox::Key::from_slice(&sha256::hash(b"").0)
            .ok_or("Sha256 produce != 32 bytes".to_string())?;

        let tag = unsafe {
            locked_buff
                .with_mut_ptr(|data| secretbox::seal_detached(data.deref_mut(), &nonce, &key))
        };

        unimplemented!();
    }
}
