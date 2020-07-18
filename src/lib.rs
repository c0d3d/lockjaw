extern crate base64;
extern crate libc;

#[macro_use]
extern crate serde;

extern crate sodiumoxide;

mod crypto;
mod databuff;
mod key;
mod keystore;
mod memlock;
mod ring;

pub use key::KeyType;
pub use keystore::LoadFailure;

use std::path::Path;

pub fn init() -> Result<(), ()> {
    sodiumoxide::init()
}

pub struct LockJaw {
    store: keystore::Keystore,
}

pub enum SecretErr<T> {
    NotFound,
    UserDefined(T),
}

pub enum SecretType {}

impl LockJaw {
    pub fn new<T: AsRef<Path>>(file_path: &T) -> Result<LockJaw, keystore::LoadFailure> {
        return Ok(LockJaw {
            store: keystore::Keystore::load(file_path)?,
        });
    }

    pub fn with_secret<S, F, Good, Bad>(&self, name: S, to_run: F) -> Result<Good, SecretErr<Bad>>
    where
        F: Fn(key::KeyType, &[u8]) -> Result<Good, Bad>,
        S: AsRef<str>,
    {
        if let k = self.store.find_key(name) {
        } else {
            return Err(SecretErr::NotFound);
        }
        unimplemented!();
    }
}
