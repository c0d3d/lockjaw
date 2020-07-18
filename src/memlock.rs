use core::pin::Pin;
use libc::__errno_location;
use libc::{c_char, c_int, c_void, mlock, munlock, strerror_r};
use std::ffi::CString;
use std::mem::size_of;
use std::ops::{Deref, Drop};

pub struct MLock<T>(Pin<Box<T>>);

impl<T> MLock<T>
where
    T: Copy + Default,
{
    pub fn new() -> Result<MLock<T>, String> {
        let x: Pin<Box<T>> = Box::pin(Default::default());
        unsafe {
            if mlock((x.deref() as *const T) as *const c_void, size_of::<T>()) != 0 {
                return Err(get_err(*__errno_location()));
            }
        }

        return Ok(MLock(x));
    }

    pub fn deref_mut(&mut self) -> Pin<&mut T> {
        return self.0.as_mut();
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for MLock<T> {
    fn as_ref(&self) -> &[u8] {
        return (*self.0).as_ref();
    }
}

impl<T> Deref for MLock<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return &*self.0;
    }
}

impl<T> Drop for MLock<T> {
    fn drop(&mut self) {
        let ptr = self.0.deref() as *const T as *const c_void;
        let size = size_of::<T>();
        unsafe { munlock(ptr, size) };
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
