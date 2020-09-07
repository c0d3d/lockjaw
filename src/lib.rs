#![feature(core_intrinsics)]
#[macro_use]
extern crate serde;

extern crate base64;
extern crate libc;
extern crate rmp_serde as rmps;
extern crate sodiumoxide;

mod crypto;
mod databuff;
mod keystore;
mod memlock;
mod ring;
mod zero;

pub use keystore::KeyType;

pub fn init<T: AsRef<std::path::Path>>(p: Option<T>) -> Result<LockJaw, LJError> {
    sodiumoxide::init().map_err(|()| LJError::SodiumOxideInitError)?;
    return LockJaw::new(p);
}

pub struct LockJaw {
    store: keystore::KeyStore,
}

pub enum LJError {
    LoadFail(String),
    SodiumOxideInitError,
    DefaultLocMissing,
}

impl From<keystore::KSLoadFail> for LJError {
    fn from(e: keystore::KSLoadFail) -> LJError {
        return LJError::LoadFail(format!("{:?}", e));
    }
}

pub enum SecretType {}

impl LockJaw {
    fn new<T: AsRef<std::path::Path>>(path: Option<T>) -> Result<LockJaw, LJError> {
        return Ok(if let Some(p) = path {
            LockJaw {
                store: keystore::KeyStore::new(p)?,
            }
        } else {
            LockJaw {
                store: keystore::KeyStore::new(format!(
                    "{}/.config/lockjaw.bin",
                    std::env::var("HOME").expect("HOME env var missing?")
                ))?,
            }
        });
    }
}
