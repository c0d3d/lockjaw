extern crate base64;
extern crate libc;
extern crate sodiumoxide;

mod crypto;
mod key;
mod keystore;
mod memlock;
mod ring;

pub fn init() -> Result<(), ()> {
    sodiumoxide::init()
}

// pub fn get_secret<T: AsRef<str>>(name: T) -> Result<
