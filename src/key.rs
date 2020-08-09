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
    tag: secretbox::Tag,
    nonce: secretbox::Nonce,
    enc_payload: memlock::MLock<DataBuff>,
}
