use crate::databuff::DataBuff;
use crate::databuff::MAX_PAYLOAD_SIZE;
use crate::memlock;
use crate::zero::explicit_bzero;
use rmps::{decode, encode};
use serde;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::hash::sha256;
use sodiumoxide::crypto::secretbox;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct KeyStoreData(HashMap<String, RawKey>);

impl Deref for KeyStoreData {
    type Target = HashMap<String, RawKey>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for KeyStoreData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

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
    InteractiveAddFailed(String),
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

        return match RawKey::create_interactive(ktype, value) {
            Ok(k) => {
                self.keys.insert(name.as_ref().to_string(), k);
                None
            }
            Err(s) => Some(KSAddFail::InteractiveAddFailed(s)),
        };
    }
}

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
        let mut key = secretbox::Key::from_slice(&sha256::hash(b"").0).unwrap();

        let ans = RawKey::create_uninteractive(ktype, value, &key);

        explicit_bzero(&mut key);

        return ans;
    }

    fn create_uninteractive(
        ktype: KeyType,
        value: &[u8],
        key: &secretbox::Key,
    ) -> Result<RawKey, String> {
        let mut locked_buff = memlock::MLock::new(
            DataBuff::from_slice(value).ok_or("Couldnt consume data".to_string())?,
        )?;

        let nonce = secretbox::gen_nonce();

        let tag = locked_buff
            .with_mut_ptr(|data| secretbox::seal_detached(data.deref_mut(), &nonce, &key));

        return Ok(RawKey {
            ktype,
            tag,
            nonce,
            enc_payload: locked_buff,
        });
    }
}
