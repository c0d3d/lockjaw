use crate::databuff::DataBuff;
use crate::memlock;
use crate::memlock::MLock;
use serde;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::secretbox;

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
    pub fn create_interactive(ktype: KeyType, value: &[u8]) -> Result<RawKey, String> {
        let mut locked_buff: memlock::MLock<DataBuff> = memlock::MLock::new()?;
        let buff = DataBuff::from_slice(value).ok_or("Couldnt consume data".to_string())?;
        locked_buff.set(buff);
        unimplemented!();
    }
}
