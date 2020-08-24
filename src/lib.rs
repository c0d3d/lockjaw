#[macro_use]
extern crate serde;

extern crate base64;
extern crate libc;
extern crate rmp_serde as rmps;
extern crate sodiumoxide;

mod crypto;
mod databuff;
mod key;
mod keystore;
mod keystore2;
mod memlock;
mod ring;

pub use key::KeyType;

pub fn init() -> Result<LockJaw, LJError> {
    sodiumoxide::init();
    return LockJaw::new();
}

pub struct LockJaw {
    store: keystore::KeyStore,
}

pub enum LJError {
    LoadFail(String),
    DefaultLocMissing,
}

impl From<keystore::KSLoadFail> for LJError {
    fn from(e: keystore::KSLoadFail) -> LJError {
        return LJError::LoadFail(format!("{:?}", e));
    }
}

pub enum SecretType {}

impl LockJaw {
    fn new() -> Result<LockJaw, LJError> {
        return if let Ok(s) = std::env::var("HOME") {
            Ok(LockJaw {
                store: keystore::KeyStore::new(format!("{}/.config/lockjaw.bin", s))?,
            })
        } else {
            Err(LJError::DefaultLocMissing)
        };
    }
}
