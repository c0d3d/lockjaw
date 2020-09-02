use libc::__errno_location;
use libc::{c_char, c_int, c_void, mlock, munlock, strerror_r};
use serde::de::{Deserializer, Error};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::mem::size_of;
use std::ops::Drop;

pub struct MLock<T>(T);

impl<T> MLock<T> {
    pub fn new(v: T) -> Result<MLock<T>, String> {
        unsafe {
            if mlock((&v as *const T) as *const c_void, size_of::<T>()) != 0 {
                return Err(get_err(*__errno_location()));
            }
        }

        return Ok(MLock(v));
    }

    pub unsafe fn with_mut_ptr<F, X>(&mut self, f: F) -> X
    where
        F: Fn(&mut T) -> X,
    {
        return f(&mut self.0);
    }
}

impl<T> Drop for MLock<T> {
    fn drop(&mut self) {
        let ptr = &self.0 as *const T as *const c_void;
        let size = size_of::<T>();
        unsafe { munlock(ptr, size) };
    }
}

impl<T> Serialize for MLock<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return self.0.serialize(serializer);
    }
}

impl<'de, T> Deserialize<'de> for MLock<T>
where
    T: Deserialize<'de> + Copy + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut locked: MLock<T> =
            MLock::new(T::deserialize(deserializer)?).map_err(D::Error::custom)?;
        return Ok(locked);
    }
}

unsafe fn get_err(num: c_int) -> String {
    let mut buff: [c_char; 512] = [0; 512];
    strerror_r(num, buff.as_mut_ptr(), 512 * size_of::<char>());
    let c = CString::from_raw(buff.as_mut_ptr());
    return c
        .into_string()
        .unwrap_or("Failed to convert error".to_string());
}
