use crate::databuff::DataBuff;
use crate::memlock::MLock;
use base64;
use serde;
use sodiumoxide::crypto::secretbox;

type StrErr<T> = Result<T, &'static str>;
type LockedBuff<T> = MLock<T>;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum KeyType {
    OTC,
    PASSWORD,
}

impl KeyType {
    fn from_string<T: AsRef<str>>(s: T) -> StrErr<KeyType> {
        return match s.as_ref() {
            "OTC" => Ok(KeyType::OTC),
            "PASSWORD" => Ok(KeyType::PASSWORD),
            _ => Err("Unrecognized key type!"),
        };
    }
}

impl Into<String> for KeyType {
    fn into(self) -> String {
        return match self {
            KeyType::OTC => "OTC",
            KeyType::PASSWORD => "PASSWORD",
        }
        .to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Key {
    name: String,
    ktype: KeyType,
    nonce: secretbox::Nonce,
    tag: secretbox::Tag,
    enc_data: LockedBuff<DataBuff>,
}

impl Key {
    pub fn read_next<T>(itr: &mut T) -> Option<Key>
    where
        T: Iterator<Item = u8>,
    {
        unimplemented!()
    }
}

impl Key {
    pub fn from_line(line: &String) -> StrErr<Key> {
        unimplemented!();
        //        return KeyStoreLine::parse(line)?.into();
    }

    // pub fn to_line(&self) -> String {
    //     let ks: KeyStoreLine<_> = self.into();
    //     return ks.into();
    // }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}
