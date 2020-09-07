use std::intrinsics::volatile_set_memory;
use std::sync::atomic::compiler_fence;
use std::sync::atomic::Ordering;

pub fn explicit_bzero<T>(v: &mut T) {
    unsafe {
        volatile_set_memory(v as *mut T, 0, 1);
    }
    compiler_fence(Ordering::Release);
}
