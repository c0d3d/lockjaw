use libc::__errno_location;
use libc::{c_char, c_int, c_void, mlock, munlock, strerror_r};
use std::ffi::CString;
use std::mem::size_of;
use std::ops::{Deref, DerefMut, Drop};

struct MLock<T>(Box<T>);

impl<T> MLock<T>
where
    T: Copy + Default,
{
    pub fn new() -> Result<MLock<T>, String> {
        let x: Box<T> = Box::new(Default::default());
        unsafe {
            if mlock((x.deref() as *const T) as *const c_void, size_of::<T>()) != 0 {
                return Err(get_err(*__errno_location()));
            }
        }

        return Ok(MLock(x));
    }
}

impl<T> Deref for MLock<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return &*self.0;
    }
}

impl<T> DerefMut for MLock<T> {
    fn deref_mut(&mut self) -> &mut T {
        return &mut *self.0;
    }
}

impl<T> Drop for MLock<T> {
    fn drop(&mut self) {
        unsafe { munlock(self.0.deref() as *const T as *const c_void, size_of::<T>()) };
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
